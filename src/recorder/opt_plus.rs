use super::*;

impl Recorder {
    pub fn record_opt_plus(&mut self, _thread: Thread, instruction: Instruction) {
        self.nodes.push(IrNode {
            type_: IrType::Internal(InternalType::I64),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![
                self.stack.pop().expect("ssa stack underflow in opt_plus"),
                self.stack.pop().expect("ssa stack underflow in opt_plus"),
            ],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
