use super::*;

impl Recorder {
    pub fn record_newarray(&mut self, _thread: Thread, instruction: Instruction) {
        self.emit(IrNode::Basic {
            type_: IrType::Yarv(ValueType::Array),
            opcode: ir::OpCode::NewArray,
            operands: vec![],
            ssa_operands: vec![],
        });
        let array = self.nodes.len() - 1;
        let count = instruction.get_operand(0);
        for _ in 0..count {
            let object = self.stack_pop();
            self.emit(IrNode::Basic {
                type_: IrType::Yarv(ValueType::Array),
                opcode: ir::OpCode::ArrayAppend,
                operands: vec![],
                ssa_operands: vec![array, object],
            });
        }
        self.stack_push(self.nodes.len() - 1);
    }
}
