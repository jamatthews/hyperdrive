use super::*;

impl Recorder {
    pub fn record_setlocal(&mut self, _thread: Thread, instruction: Instruction) {
        let offset = instruction.get_operand(0);
        let popped = self.stack.pop().expect("ssa stack underflow in setlocal");

        self.nodes.push(IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![offset],
            ssa_operands: vec![popped],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
