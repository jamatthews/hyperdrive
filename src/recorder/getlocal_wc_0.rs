use super::*;

impl Recorder {
    pub fn record_getlocal(&mut self, thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let value: Value = thread.get_local(offset).into();

        let offset_from_base_bp = self.ep - (offset as isize * 8);

        let ssa_ref = match self.stack.get(&offset_from_base_bp) {
            Some(ssa_ref) => *ssa_ref,
            None => {
                let type_ = match value.type_() {
                    ValueType::Fixnum => IrType::Internal(InternalType::I64),
                    x => IrType::Yarv(x),
                };
                self.emit(IrNode::Basic {
                    type_: type_,
                    opcode: ir::OpCode::StackLoad(offset_from_base_bp),
                    operands: vec![],
                    ssa_operands: vec![],
                });
                self.stack.insert(offset_from_base_bp, self.nodes.len() - 1);
                self.nodes.len() - 1
            }
        };
        self.stack_push(ssa_ref);
    }
}
