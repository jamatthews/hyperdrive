use super::*;

impl Recorder {
    pub fn record_opt_plus(&mut self, _thread: Thread, instruction: Instruction) {
        let a = self.stack_pop();
        let b = self.stack_pop();

        self.emit(IrNode::Basic {
            type_: IrType::Internal(InternalType::I64),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![a, b],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
