extern crate cranelift;
extern crate cranelift_codegen;
extern crate cranelift_module;
extern crate cranelift_simplejit;

extern crate hyperdrive_ruby;

#[cfg(cargo_c)]
mod capi;
mod compiler;
mod ir;
mod trace;
mod yarv_opcode;

#[cfg(cargo_c)]
pub use capi::*;
use trace::Trace;

static mut CURRENT_TRACE: Option<Trace> = None;
