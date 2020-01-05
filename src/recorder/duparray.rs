use super::*;

impl Recorder {
    pub fn record_duparray(&mut self, _thread: Thread, instruction: Instruction) {
        let array = instruction.get_operand(0);
        self.emit(IrNode::Basic {
            type_: IrType::Yarv(ValueType::Array),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![array],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
