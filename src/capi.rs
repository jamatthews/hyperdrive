#[cfg(cargo_c)]

use hyperdrive_ruby::rb_control_frame_t;
use hyperdrive_ruby::rb_thread_t;
use hyperdrive_ruby::rb_vm_insn_addr2insn;
use hyperdrive_ruby::VALUE;

use CURRENT_TRACE;
use trace::Trace;

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_record_instruction(
    _thread: *const rb_thread_t,
    _cfp: *const rb_control_frame_t,
    pc: *const VALUE,
) -> i32
{
    let opcode: i32 = rb_vm_insn_addr2insn(*pc as *const _);
    match &mut CURRENT_TRACE {
        Some(trace) => {
            if *pc == trace.anchor {
                // TODO compile trace and insert trace instruction
                CURRENT_TRACE = None;
                return 1;
            } else {
                trace.add_opcode(std::mem::transmute(opcode));
            }
        },
        None => panic!("No trace started"),
    };
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_begin_trace(
    _thread: *const rb_thread_t,
    _cfp: *const rb_control_frame_t,
    pc: *const VALUE,
) {
    let trace = Trace {
        opcodes: vec![],
        anchor: *pc,
    };
    match &mut CURRENT_TRACE {
        Some(_) => panic!("trace already recording"),
        _ => CURRENT_TRACE = Some(trace),
    };
}
