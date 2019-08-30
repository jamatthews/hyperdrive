use hyperdrive_ruby::rb_method_type_t;
use hyperdrive_ruby::VALUE;
use hyperdrive_ruby::rb_call_cache;



pub struct VmCallCache {
    pub cc: *const rb_call_cache,
}

impl VmCallCache {
    pub fn new(cc: *const rb_call_cache) -> Self {
        Self {
            cc: cc,
        }
    }

    pub fn get_func(&self) -> unsafe extern "C" fn() -> VALUE {
        unsafe {
            let method_entry = *(*self.cc).me;
            let definition = *method_entry.def;
            definition.body.cfunc.func.expect("missing func in callcache")
        }
    }

    pub fn get_type(&self) -> rb_method_type_t {
        unsafe {
            let method_entry = *(*self.cc).me;
            let definition = *method_entry.def;
            definition.type_()
        }
    }
}
