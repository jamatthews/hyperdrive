use super::*;

impl Recorder {
    pub fn record_opt_empty_p(&mut self, _thread: Thread, instruction: Instruction) {
        self.nodes.push(IrNode {
            type_: IrType::Internal(InternalType::Bool),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![self.stack.pop().expect("ssa stack underflow in opt_empty_p")],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
