use super::*;

impl Recorder {
    pub fn record_putstring(&mut self, _thread: Thread, instruction: Instruction) {
        let raw_value = instruction.get_operand(0);

        let ssa_ref = self.emit(IrNode::Constant {
            type_: IrType::Yarv(ValueType::RString),
            reference: raw_value,
        });
        self.stack_push(ssa_ref);
    }
}
