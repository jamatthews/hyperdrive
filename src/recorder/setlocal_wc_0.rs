use super::*;

impl Recorder {
    pub fn record_setlocal(&mut self, _thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let popped = self.stack_pop();

        self.nodes.push(IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![offset],
            ssa_operands: vec![popped],
        });
    }
}
