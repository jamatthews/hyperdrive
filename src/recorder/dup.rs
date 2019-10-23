use super::*;

impl Recorder {
    pub fn record_dup(&mut self, _thread: Thread, instruction: Instruction) {
        let popped = self.stack.pop().expect("ssa stack underflow in dup");
        self.nodes.push(IrNode {
            type_: self.nodes[popped].type_.clone(),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![popped],
        });
        self.stack.push(self.nodes.len() - 1);
        self.stack.push(self.nodes.len() - 1);
    }
}
