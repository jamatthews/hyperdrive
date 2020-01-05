use super::*;

impl Recorder {
    pub fn record_newhash(&mut self, _thread: Thread, instruction: Instruction) {
        self.emit(IrNode::Basic {
            type_: IrType::Yarv(ValueType::Hash),
            opcode: ir::OpCode::NewHash,
            operands: vec![],
            ssa_operands: vec![],
        });
        let hash = self.nodes.len() - 1;
        let count = instruction.get_operand(0);
        for _ in 0..(count / 2) {
            let value = self.stack_pop();
            let key = self.stack_pop();
            self.emit(IrNode::Basic {
                type_: IrType::Yarv(ValueType::Hash),
                opcode: ir::OpCode::HashSet,
                operands: vec![],
                ssa_operands: vec![hash, key, value],
            });
        }
        self.stack_push(self.nodes.len() - 1);
    }
}
