use super::*;

use hyperdrive_ruby::rb_method_type_t_VM_METHOD_TYPE_CFUNC;

impl Recorder {
    pub fn record_opt_send_without_block(&mut self, thread: Thread, instruction: Instruction) {
        let receiver = self.stack_pop();
        let call_cache = CallCache::new(instruction.get_operand(1) as *const _);
        let snapshot = self.snapshot(thread);

        if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
            self.nodes.push(IrNode {
                type_: IrType::Internal(InternalType::Value),
                opcode: ir::OpCode::Yarv(instruction.opcode()),
                operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                ssa_operands: vec![receiver],
            });
            self.stack_push(self.nodes.len() - 1);
        } else {
            self.nodes.push(IrNode {
                type_: IrType::None,
                opcode: ir::OpCode::Snapshot(snapshot),
                operands: vec![],
                ssa_operands: vec![],
            });
            self.stack_push(receiver);
        }
    }
}
