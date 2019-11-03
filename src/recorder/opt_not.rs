use super::*;

impl Recorder {
    pub fn record_opt_not(&mut self, _thread: Thread, instruction: Instruction) {
        let popped = self.stack_pop();
        self.nodes.push(IrNode {
            type_: IrType::Internal(InternalType::Bool),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![popped],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
