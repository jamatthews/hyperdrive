#[cfg(cargo_c)]
use hyperdrive_ruby::rb_control_frame_t;
use hyperdrive_ruby::rb_thread_t;
use hyperdrive_ruby::VALUE;

use HYPERDRIVE;
use Mode;

extern "C" {
    #[no_mangle]
    static mut trace_recording: i32;
    #[no_mangle]
    static mut trace_dispatch: unsafe extern "C" fn(*const rb_thread_t, *const rb_control_frame_t, *const VALUE);
    #[no_mangle]
    static mut record_instruction: unsafe extern "C" fn(*const rb_thread_t, *const rb_control_frame_t, *const VALUE);
}

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_init(){
    trace_dispatch = hyperdrive_trace_dispatch;
    record_instruction = hyperdrive_record_instruction;
}

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_trace_dispatch(
    _thread: *const rb_thread_t,
    _cfp: *const rb_control_frame_t,
    pc: *const VALUE,
) {
    ::trace_dispatch(pc as u64);
    match &HYPERDRIVE.lock().unwrap().mode {
        Mode::Recording(_) => { trace_recording = 1 },
        _ => { trace_recording = 0  },
    };
}

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_record_instruction(
    _thread: *const rb_thread_t,
    _cfp: *const rb_control_frame_t,
    pc: *const VALUE,
) {
    ::trace_record_instruction(pc);
    match &HYPERDRIVE.lock().unwrap().mode {
        Mode::Recording(_) => { trace_recording = 1 },
        _ => { trace_recording = 0  },
    };
}

#[no_mangle]
pub unsafe extern "C" fn hyperdrive_stop_recording() {
    trace_recording = 0;
}

#[no_mangle]
pub extern "C" fn hyperdrive_recording() -> i64 {
    match &HYPERDRIVE.lock().unwrap().mode {
        Mode::Recording(_) => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn hyperdrive_trace_count() -> usize {
    HYPERDRIVE.lock().unwrap().trace_heads.keys().len()
}
