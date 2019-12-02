use super::*;

impl Recorder {
    pub fn record_getlocal(&mut self, thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let value: Value = thread.get_local(offset).into();

        let ssa_ref = match self.stack.get(&(offset as isize * -8)) {
            Some(ssa_ref) => *ssa_ref,
            None => {
                let type_ = match value.type_() {
                    ValueType::Fixnum => IrType::Internal(InternalType::I64),
                    x => IrType::Yarv(x),
                };
                self.nodes.push(IrNode {
                    type_: type_,
                    opcode: ir::OpCode::Yarv(instruction.opcode()),
                    operands: vec![offset],
                    ssa_operands: vec![],
                });
                self.stack.insert(offset as isize * -8, self.nodes.len() - 1);
                self.nodes.len() - 1
            }
        };
        self.stack_push(ssa_ref);
    }
}
