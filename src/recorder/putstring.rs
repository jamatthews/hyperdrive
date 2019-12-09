use super::*;

impl Recorder {
    pub fn record_putstring(&mut self, _thread: Thread, instruction: Instruction) {
        let raw_value = instruction.get_operand(0);

        self.nodes.push(IrNode::Basic {
            type_: IrType::Yarv(ValueType::RString),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![raw_value],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
