use super::*;

impl Recorder {
    pub fn record_putobject_fix(&mut self, _thread: Thread, instruction: Instruction) {
        self.emit(IrNode::Basic {
            type_: IrType::Internal(InternalType::I64),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
