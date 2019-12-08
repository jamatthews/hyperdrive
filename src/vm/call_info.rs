use hyperdrive_ruby::rb_call_info;

pub struct CallInfo {
    ci: *const rb_call_info,
}

impl CallInfo {
    pub fn new(ci: *const rb_call_info) -> Self {
        Self { ci: ci }
    }

    pub fn get_orig_argc(&self) -> i32 {
        unsafe { (*self.ci).orig_argc }
    }
}
