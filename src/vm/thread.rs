use hyperdrive_ruby::rb_control_frame_struct;
use hyperdrive_ruby::rb_iseq_struct;
use hyperdrive_ruby::rb_thread_t;
use hyperdrive_ruby::VALUE;

#[derive(Clone)]
pub struct Thread {
    thread: *const rb_thread_t,
}

impl Thread {
    pub fn new(thread: *const rb_thread_t) -> Self {
        Self { thread: thread }
    }

    pub fn get_thread_ptr(&self) -> *const u64 {
        self.thread as *const u64
    }

    pub fn get_cf(&self) -> rb_control_frame_struct {
        unsafe { *(*(*self.thread).ec).cfp }
    }

    pub fn get_prev_cf(&self) -> rb_control_frame_struct {
        unsafe { *(*(*self.thread).ec).cfp.offset(1) }
    }

    pub fn get_pc(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).pc }
    }

    pub fn set_pc(&self, target: u64) {
        unsafe { (*(*(*self.thread).ec).cfp).pc = target as *const u64 };
    }

    pub fn get_sp(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).sp }
    }

    pub fn get_ep(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).ep }
    }

    pub fn set_ep(&self, target: u64) {
        unsafe { (*(*(*self.thread).ec).cfp).ep = target as *mut u64 };
    }

    pub fn set_sp(&self, target: u64) {
        unsafe { (*(*(*self.thread).ec).cfp).sp = target as *mut u64 };
    }

    pub fn get_bp(&self) -> *const VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).bp }
    }

    pub fn set_bp(&self, target: u64) {
        unsafe { (*(*(*self.thread).ec).cfp).bp = target as *mut u64 };
    }

    pub fn get_sp_ptr(&self) -> *const *mut VALUE {
        unsafe { (&(*(*(*self.thread).ec).cfp).sp) as *const _ }
    }

    pub fn get_self(&self) -> VALUE {
        unsafe { (*(*(*self.thread).ec).cfp).self_ }
    }

    pub fn get_local(&self, offset: u64) -> VALUE {
        unsafe { *self.get_ep().offset(-(offset as isize)) }
    }

    pub fn get_iseq(&self) -> *const rb_iseq_struct {
        unsafe { (*(*(*self.thread).ec).cfp).iseq }
    }

    pub fn set_iseq(&self, target: *const rb_iseq_struct) {
        unsafe { (*(*(*self.thread).ec).cfp).iseq = target };
    }

    pub fn push_frame(&self) {
        unsafe {
            let new_frame = (*(*self.thread).ec).cfp.offset(-1);
            *new_frame = (*(*(*self.thread).ec).cfp).clone();
            (*(*self.thread).ec).cfp = new_frame;
        }
    }
}
