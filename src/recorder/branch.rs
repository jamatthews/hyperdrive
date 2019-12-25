use super::*;

impl Recorder {
    pub fn record_branch(&mut self, thread: Thread, _instruction: Instruction) {
        let value: Value = unsafe { *thread.get_sp().offset(-1) }.into();
        let snap = self.snapshot();
        let popped = self.stack_pop();
        self.nodes.push(IrNode::Guard {
            type_: ir::IrType::Yarv(value.type_()),
            snap: snap,
            ssa_operands: vec![popped],
        });
        ;
    }
}
