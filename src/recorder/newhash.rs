use super::*;

impl Recorder {
    pub fn record_newhash(&mut self, _thread: Thread, instruction: Instruction) {
        self.nodes.push(IrNode {
            type_: IrType::Yarv(ValueType::Hash),
            opcode: ir::OpCode::NewHash,
            operands: vec![],
            ssa_operands: vec![],
        });
        let hash = self.nodes.len() - 1;
        let count = instruction.get_operand(0);
        for _ in 0..(count / 2) {
            let value = self.stack.pop().expect("stack underflow recording newhash");
            let key = self.stack.pop().expect("stack underflow recording newhash");
            self.nodes.push(IrNode {
                type_: IrType::Yarv(ValueType::Hash),
                opcode: ir::OpCode::HashSet,
                operands: vec![],
                ssa_operands: vec![hash, key, value],
            });
        }
        self.stack.push(self.nodes.len() - 1);
    }
}
