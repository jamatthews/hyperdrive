use super::*;

impl Recorder {
    pub fn record_comparison(&mut self, _thread: Thread, instruction: Instruction) {
        let b = self.stack.pop().expect("ssa stack underflow in opt_lt");
        let a = self.stack.pop().expect("ssa stack underflow in opt_lt");
        self.nodes.push(
            IrNode {
                type_: IrType::Internal(InternalType::Bool),
                opcode: ir::OpCode::Yarv(instruction.opcode()),
                operands: vec![],
                ssa_operands: vec![a,b],
            }
        );
        self.stack.push(self.nodes.len() - 1);
    }
}
