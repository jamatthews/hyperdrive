use super::*;

impl Recorder {
    pub fn record_putself(&mut self, thread: Thread, instruction: Instruction) {
        let raw_value = thread.get_self();
        let value: Value = raw_value.into();

        self.nodes.push(IrNode {
            type_: IrType::Yarv(value.type_()),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
