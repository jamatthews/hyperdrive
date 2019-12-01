use super::*;

impl Recorder {
    pub fn record_setlocal(&mut self, _thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let popped = self.stack_pop();

        self.stack.insert(offset as isize * -8, popped);
    }
}
