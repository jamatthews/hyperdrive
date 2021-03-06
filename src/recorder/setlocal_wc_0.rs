use super::*;

impl Recorder {
    pub fn record_setlocal(&mut self, _thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let popped = self.stack_pop();
        let offset_from_base_bp = self.call_stack.last().unwrap().ep - (offset as isize * 8);

        self.stack.insert(offset_from_base_bp, popped);
    }
}
