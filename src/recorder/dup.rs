use super::*;

impl Recorder {
    pub fn record_dup(&mut self, _thread: Thread, instruction: Instruction) {
        let popped = self.stack_pop();
        self.emit(IrNode::Basic {
            type_: self.nodes[popped].type_(),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![popped],
        });
        self.stack_push(self.nodes.len() - 1);
        self.stack_push(self.nodes.len() - 1);
    }
}
