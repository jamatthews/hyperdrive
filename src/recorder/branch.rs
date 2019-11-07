use super::*;

impl Recorder {
    pub fn record_branch(&mut self, thread: Thread, _instruction: Instruction) {
        let value: Value = unsafe { *thread.get_sp().offset(-1) }.into();
        let popped = self.stack_pop();
        let snapshot = self.snapshot(thread);
        self.nodes.push(IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Guard(IrType::Yarv(value.type_()), snapshot),
            operands: vec![],
            ssa_operands: vec![popped],
        });
        ;
    }
}
