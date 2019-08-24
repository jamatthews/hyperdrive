extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_module;
extern crate cranelift_simplejit;
#[macro_use]
extern crate lazy_static;

extern crate hyperdrive_ruby;

#[cfg(cargo_c)]
mod capi;
mod instruction_recorder;
mod ir;
mod trace;
mod trace_compiler;
mod yarv_instruction;
mod yarv_opcode;
mod vm_thread;


use std::collections::HashMap;
use std::sync::Mutex;

use hyperdrive_ruby::VALUE;

#[cfg(cargo_c)]
pub use capi::*;
pub use trace::Trace;
use vm_thread::VmThread;
use ir::OpCode;

lazy_static! {
    static ref HYPERDRIVE: Mutex<Hyperdrive> = Mutex::new(
        Hyperdrive { mode: Mode::Normal, counters: HashMap::new(), trace_heads: HashMap::new() }
    );
}

struct Hyperdrive {
    pub mode: Mode,
    pub counters: HashMap<u64,u64>,
    pub trace_heads: HashMap<u64,Trace>,
}


pub enum Mode {
    Normal,
    Recording(Trace),
    Executing,
}

// trace dispatch and either enter a trace or start recording a trace
fn trace_dispatch(thread: VmThread) {
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Normal => {
            let pc = thread.get_pc() as u64;
            if let Some(existing_trace) = hyperdrive.trace_heads.get(&pc) {
                let target_pc = match existing_trace.nodes.last().unwrap().opcode {
                    OpCode::Snapshot(pc) => pc as *const VALUE,
                    _ => panic!("tried to exit without a snapshot"),
                };
                existing_trace.compiled_code.unwrap()(thread.get_ep());
                thread.set_pc(target_pc);
            } else {
                *hyperdrive.counters.entry(pc).or_insert(0) += 1;
                let count = hyperdrive.counters.get(&pc).unwrap();
                if *count > 1000 {
                    hyperdrive.mode = Mode::Recording(Trace::new(pc));
                }
            }
        },
        _ => {},
    }
}

// recording may terminate, triggering compilation
fn trace_record_instruction(thread: VmThread){
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Recording(trace) if thread.get_pc() as u64 == trace.start => {
            trace.complete();
            let mut trace = trace.clone();
            trace.compile();
            hyperdrive.trace_heads.insert(trace.start, trace);
            hyperdrive.mode = Mode::Normal;
        },
        Mode::Recording(trace) => trace.record_instruction(thread),
        _ => panic!("tried to record instruction while not recording trace")
    };
}
