use super::*;

impl Recorder {
    pub fn record_putobject(&mut self, _thread: Thread, instruction: Instruction) {
        let raw_value = instruction.get_operand(0);
        let value: Value = raw_value.into();

        let type_ = if value.type_() == ValueType::Fixnum {
            IrType::Internal(InternalType::I64)
        } else {
            IrType::Yarv(value.type_())
        };

        self.nodes.push(IrNode {
            type_: type_,
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![raw_value],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
