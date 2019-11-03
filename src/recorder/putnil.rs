use super::*;


impl Recorder {
    pub fn record_putnil(&mut self,
        _thread: Thread,
        instruction: Instruction,
    ) {
        self.nodes.push(IrNode {
            type_: IrType::Yarv(ValueType::Nil),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
