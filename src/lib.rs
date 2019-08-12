extern crate hyperdrive_ruby;

#[cfg(cargo_c)]
mod capi;
mod opcode;
mod trace;

#[cfg(cargo_c)]
pub use capi::*;
use trace::Trace;

static mut CURRENT_TRACE: Option<Trace> = None;
