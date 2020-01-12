extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_module;
extern crate cranelift_simplejit;
#[macro_use]
extern crate lazy_static;

extern crate hyperdrive_ruby;

#[cfg(cargo_c)]
mod capi;
mod compiler;
mod ir;
mod recorder;
mod trace;
mod vm;

use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::isa::CallConv;
use cranelift_module::*;
use cranelift_simplejit::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Mutex;

use hyperdrive_ruby::*;

#[cfg(cargo_c)]
pub use capi::*;
use ir::*;
pub use recorder::*;
pub use trace::*;
pub use vm::*;

lazy_static! {
    static ref HYPERDRIVE: Mutex<Hyperdrive> = {
        let sig = Signature {
            params: vec![AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };
        let sig2 = Signature {
            params: vec![AbiParam::new(I64), AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };
        let sig3 = Signature {
            params: vec![],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };
        let sig4 = Signature {
            params: vec![AbiParam::new(I64), AbiParam::new(I64), AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };

        let mut simplejit = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        simplejit.symbol("_rb_ary_resurrect", rb_ary_resurrect as *const u8);
        simplejit.symbol("_rb_ary_push", rb_ary_push as *const u8);
        simplejit.symbol("_rb_ary_new", rb_ary_new as *const u8);
        simplejit.symbol("_rb_ary_aref1", rb_ary_aref1 as *const u8);
        simplejit.symbol("_rb_ary_store", rb_ary_store as *const u8);
        simplejit.symbol("_rb_hash_aref", rb_hash_aref as *const u8);
        simplejit.symbol("_rb_hash_aset", rb_hash_aset as *const u8);
        simplejit.symbol("_rb_hash_new", rb_hash_new as *const u8);
        simplejit.symbol("_rb_str_strlen", rb_str_strlen as *const u8);
        simplejit.symbol("_rb_ary_len", rb_ary_len as *const u8);

        let mut module = Module::new(simplejit);
        module
            .declare_function("_rb_ary_resurrect", Linkage::Import, &sig)
            .unwrap();
        module.declare_function("_rb_ary_push", Linkage::Import, &sig2).unwrap();
        module.declare_function("_rb_ary_new", Linkage::Import, &sig3).unwrap();
        module
            .declare_function("_rb_ary_aref1", Linkage::Import, &sig2)
            .unwrap();
        module
            .declare_function("_rb_ary_store", Linkage::Import, &sig4)
            .unwrap();
        module
            .declare_function("_rb_hash_aref", Linkage::Import, &sig2)
            .unwrap();
        module
            .declare_function("_rb_hash_aset", Linkage::Import, &sig4)
            .unwrap();
        module.declare_function("_rb_hash_new", Linkage::Import, &sig3).unwrap();
        module
            .declare_function("_rb_str_strlen", Linkage::Import, &sig)
            .unwrap();
        module.declare_function("_rb_ary_len", Linkage::Import, &sig).unwrap();

        Mutex::new(Hyperdrive {
            mode: Mode::Normal,
            counters: HashMap::new(),
            failures: HashMap::new(),
            trace_heads: HashMap::new(),
            module: module,
        })
    };
}

struct Hyperdrive {
    pub mode: Mode,
    pub counters: HashMap<u64, u64>,
    pub failures: HashMap<u64, u64>,
    pub trace_heads: HashMap<u64, Rc<Pin<Box<Trace>>>>,
    pub module: Module<SimpleJITBackend>,
}

unsafe impl Send for Hyperdrive {}

pub enum Mode {
    Normal,
    Recording(Recorder),
    Executing,
}

fn trace_dispatch(thread: Thread) {
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Normal => {
            let pc = thread.get_pc() as u64;
            if let Some(existing_trace) = hyperdrive.trace_heads.get(&pc) {
                let trace_function = existing_trace.compiled_code.unwrap();
                let base_bp = thread.get_bp() as u64;
                let exit_node: *const IrNode =
                    trace_function(thread.get_thread_ptr(), thread.get_bp(), thread.get_self());
                let exit_node = unsafe { &*exit_node };
                let (snap, exit_count, error_count) = match exit_node {
                    IrNode::Guard { snap, exit_count, error_count, .. } => (snap, exit_count, error_count),
                    _ => panic!("exit node not a guard {}"),
                };

                for (i, frame) in snap.call_stack.iter().enumerate() {
                    thread.set_bp(base_bp + frame.bp as u64);
                    thread.set_sp(base_bp + frame.sp as u64);
                    thread.set_ep(base_bp + frame.ep as u64);
                    thread.set_iseq(frame.iseq);

                    if i < snap.call_stack.len() - 1 {
                        thread.set_pc(frame.pc);
                        thread.push_frame();
                    } else {
                        thread.set_pc(frame.pc - 8);
                    }
                }

                if exit_count.get() > 5 && error_count.get() < 3 {
                    let offset = existing_trace.nodes.iter().position(|x| x == exit_node).unwrap();
                    hyperdrive.mode = Mode::Recording(Recorder::new(thread, Some((Rc::clone(&existing_trace), offset))));
                } else {
                    exit_count.set(exit_count.get() + 1);
                }
            } else {
                *hyperdrive.counters.entry(pc).or_insert(0) += 1;
                let count = hyperdrive.counters.get(&pc).unwrap();
                if *count > 1000 {
                    let failures = hyperdrive.failures.get(&pc).unwrap_or(&0);
                    if *failures < 5 {
                        hyperdrive.mode = Mode::Recording(Recorder::new(thread, None));
                    }
                }
            }
        }
        _ => {}
    }
}

fn trace_record_instruction(thread: Thread) {
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Recording(recorder) => match recorder.record_instruction(thread.clone()) {
            Ok(true) => {
                let mut trace = Pin::new(Box::new(Trace::new(recorder.nodes.clone(), thread.clone())));
                match &recorder.parent {
                    Some((trace, offset)) => {
                        match trace.nodes.get(*offset).unwrap() {
                            IrNode::Guard { error_count, .. } => {
                                println!("error recording side trace");
                                error_count.set(error_count.get() + 1);
                            },
                            _ => {},
                        }
                    },
                    None => {
                        trace.compile(&mut hyperdrive.module);
                        hyperdrive.trace_heads.insert(trace.anchor, Rc::new(trace));
                    }
                }
                hyperdrive.mode = Mode::Normal;
            }
            Err(err) => {
                println!("Trace Recording Aborted: {}", err);
                let pc = recorder.anchor.clone();
                *hyperdrive.failures.entry(pc).or_insert(0) += 1;
                hyperdrive.mode = Mode::Normal;
            }
            _ => {}
        },
        _ => panic!("tried to record instruction while not recording trace"),
    };
}

fn trace_exit(_pc: u64) {}

use hyperdrive_ruby::ruby_rarray_flags_RARRAY_EMBED_FLAG;
use hyperdrive_ruby::ruby_rarray_flags_RARRAY_EMBED_LEN_MASK;
use hyperdrive_ruby::ruby_rarray_flags_RARRAY_EMBED_LEN_SHIFT;
use hyperdrive_ruby::RArray;
use std::os::raw::c_long;

#[no_mangle]
pub unsafe extern "C" fn rb_ary_len(value: vm::Value) -> c_long {
    let rarray: *const RArray = std::mem::transmute(value.value);
    let flags = (*rarray).basic.flags;

    if flags & (ruby_rarray_flags_RARRAY_EMBED_FLAG as u64) == 0 {
        (*rarray).as_.heap.len
    } else {
        ((flags as i64 >> ruby_rarray_flags_RARRAY_EMBED_LEN_SHIFT as i64)
            & (ruby_rarray_flags_RARRAY_EMBED_LEN_MASK as i64 >> ruby_rarray_flags_RARRAY_EMBED_LEN_SHIFT as i64))
            as c_long
    }
}
