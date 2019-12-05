use super::*;

impl Recorder {
    pub fn record_putself(&mut self, _thread: Thread, _instruction: Instruction) {
        let self_ = *self.call_stack.last().expect("stack underflow");
        self.stack_push(self_);
    }
}
