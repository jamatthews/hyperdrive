extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_module;
extern crate cranelift_simplejit;
#[macro_use]
extern crate lazy_static;

extern crate hyperdrive_ruby;

#[cfg(cargo_c)]
mod capi;
mod ir;
mod trace;
mod trace_compiler;
mod trace_recorder;
mod yarv_instruction;
mod yarv_opcode;
mod yarv_types;
mod vm_call_cache;
mod vm_thread;

use cranelift::prelude::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::ir::types::I64;
use cranelift_module::*;
use cranelift_simplejit::*;
use std::collections::HashMap;
use std::sync::Mutex;

use hyperdrive_ruby::*;

#[cfg(cargo_c)]
pub use capi::*;
pub use trace::Trace;
pub use trace_recorder::TraceRecorder;
use vm_thread::VmThread;
use ir::OpCode;


lazy_static! {
    static ref HYPERDRIVE: Mutex<Hyperdrive> = {
        let sig = Signature {
            params: vec![AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };

        let mut simplejit = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        simplejit.symbol("_rb_ary_resurrect", rb_ary_resurrect as *const u8);
        simplejit.symbol("_rb_str_strlen", rb_str_strlen as *const u8);
        let mut module = Module::new(simplejit);
        module.declare_function("_rb_ary_resurrect", Linkage::Import, &sig).unwrap();
        module.declare_function("_rb_str_strlen", Linkage::Import, &sig).unwrap();

        Mutex::new(
            Hyperdrive { mode: Mode::Normal, counters: HashMap::new(), trace_heads: HashMap::new(), module: module  }
        )
    };
}


struct Hyperdrive {
    pub mode: Mode,
    pub counters: HashMap<u64,u64>,
    pub trace_heads: HashMap<u64,Trace>,
    pub module: Module<SimpleJITBackend>,
}

unsafe impl Send for Hyperdrive {}

pub enum Mode {
    Normal,
    Recording(TraceRecorder),
    Executing,
}

fn trace_dispatch(thread: VmThread) {
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Normal => {
            let pc = thread.get_pc() as u64;
            if let Some(existing_trace) = hyperdrive.trace_heads.get(&pc) {
                let target_pc = match existing_trace.nodes[existing_trace.nodes.len() - 2].opcode {
                    OpCode::Snapshot(pc) => pc as *const VALUE,
                    _ => panic!("tried to exit without a snapshot"),
                };
                let trace_function = existing_trace.compiled_code.unwrap();
                trace_function(thread.get_ep());
                thread.set_pc(target_pc);
            } else {
                *hyperdrive.counters.entry(pc).or_insert(0) += 1;
                let count = hyperdrive.counters.get(&pc).unwrap();
                if *count > 1000 {
                    hyperdrive.mode = Mode::Recording(TraceRecorder::new(pc));
                }
            }
        },
        _ => {},
    }
}

fn trace_record_instruction(thread: VmThread){
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Recording(recorder) => {
            recorder.record_instruction(thread);
            if recorder.complete {
                let mut trace = Trace::new(recorder.anchor, recorder.nodes.clone());
                trace.compile(&mut hyperdrive.module);
                hyperdrive.trace_heads.insert(trace.anchor, trace);
                hyperdrive.mode = Mode::Normal;
            }
        },
        _ => panic!("tried to record instruction while not recording trace")
    };
}
