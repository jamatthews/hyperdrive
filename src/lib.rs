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
mod trace;
mod yarv_opcode;

use yarv_opcode::YarvOpCode;
use std::collections::HashMap;
use std::sync::Mutex;

use hyperdrive_ruby::rb_vm_insn_addr2insn;
use hyperdrive_ruby::VALUE;

#[cfg(cargo_c)]
pub use capi::*;
pub use trace::Trace;

lazy_static! {
    static ref HYPERDRIVE: Mutex<Hyperdrive> = Mutex::new(
        Hyperdrive { mode: Mode::Normal, counters: HashMap::new(), trace_heads: HashMap::new() }
    );
}

struct Hyperdrive{
    pub mode: Mode,
    pub counters: HashMap<u64,u64>,
    pub trace_heads: HashMap<u64,Trace>,
}


pub enum Mode {
    Normal,
    Recording(Trace),
    Executing,
}

fn trace_dispatch(pc: u64) {
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Normal => {
            if let Some(existing_trace) = hyperdrive.trace_heads.get(&pc) {
                existing_trace.compiled_code.unwrap()();
            } else {
                *hyperdrive.counters.entry(pc).or_insert(0) += 1;
                let count = hyperdrive.counters.get(&pc).unwrap();
                if *count > 1000 {
                    let new_trace = Trace {
                        anchor: pc,
                        nodes: vec![],
                        compiled_code: None,
                    };
                    hyperdrive.mode = Mode::Recording(new_trace);
                }
            }
        },
        Mode::Recording(trace) => {
            if pc == trace.anchor {
                let mut trace = trace.clone();
                //trace.compile();
                hyperdrive.trace_heads.insert(trace.anchor, trace);
                hyperdrive.mode = Mode::Normal;
            }
        },
        _ => {},
    }
}

fn trace_record_instruction(pc: *const VALUE){
    let raw_opcode: i32 = unsafe { rb_vm_insn_addr2insn(*pc as *const _) };
    let parsed_opcode: YarvOpCode = unsafe { std::mem::transmute(raw_opcode) };
    let hyperdrive = &mut HYPERDRIVE.lock().unwrap();
    match &mut hyperdrive.mode {
        Mode::Recording(trace) => {
            trace.add_node(parsed_opcode);
        },
        _ => panic!("tried to record instruction while not recording trace")
    };
}
