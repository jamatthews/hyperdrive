use super::*;

impl Recorder {
    pub fn record_putstring(&mut self, _thread: Thread, instruction: Instruction) {
        let raw_value = instruction.get_operand(0);

        self.nodes.push(IrNode {
            type_: IrType::Yarv(ValueType::RString),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![raw_value],
            ssa_operands: vec![],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
