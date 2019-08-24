use hyperdrive_ruby::VALUE;
use hyperdrive_ruby::rb_thread_t;

pub struct VmThread {
    thread: *mut rb_thread_t,
}

impl VmThread {
    pub fn get_pc(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).pc }
    }

    pub fn get_sp(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).sp }
    }

    pub fn get_ep(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).ep }
    }
}
