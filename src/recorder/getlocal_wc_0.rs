use super::*;

impl Recorder {
    pub fn record_getlocal(&mut self, thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let value: Value = thread.get_local(offset).into();
        self.nodes.push(IrNode {
            type_: IrType::Yarv(value.type_()),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![offset],
            ssa_operands: vec![],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
