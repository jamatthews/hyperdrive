use hyperdrive_ruby::rb_thread_t;
use hyperdrive_ruby::VALUE;

#[derive(Clone)]
pub struct Thread {
    thread: *const rb_thread_t,
}

impl Thread {
    pub fn new(thread: *const rb_thread_t) -> Self {
        Self {
            thread: thread,
        }
    }

    pub fn get_pc(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).pc }
    }

    pub fn set_pc(&self, target_pc: *const u64) {
        unsafe { (*(*(*self.thread).ec).cfp).pc  = target_pc };
    }

    pub fn get_sp(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).sp }
    }

    pub fn get_ep(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).ep }
    }

    pub fn get_self(&self) -> VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).self_ }
    }

    pub fn get_local(&self, offset: u64) -> VALUE {
        unsafe { * self.get_ep().offset(-(offset as isize)) }
    }
}
