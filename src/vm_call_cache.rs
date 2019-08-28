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
            let body = definition.body;
            let cfunc = body.cfunc;
            //println!("in call cache: {:?}", cfunc.as_ref().func.expect("missing func"));
            cfunc.as_ref().func.expect("missing func")
        }
    }
}
