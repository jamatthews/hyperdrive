use super::*;

use hyperdrive_ruby::rb_method_type_t_VM_METHOD_TYPE_CFUNC;

impl Recorder {
    pub fn record_opt_send_without_block(&mut self, thread: Thread, instruction: Instruction) {
        let receiver = self.stack
            .pop()
            .expect("ssa self.stack underflow in opt_send_without_block");
        let call_cache = CallCache::new(instruction.get_operand(1) as *const _);

        if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
            self.nodes.push(IrNode {
                type_: IrType::Internal(InternalType::Value),
                opcode: ir::OpCode::Yarv(instruction.opcode()),
                operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                ssa_operands: vec![receiver],
            });
        } else {
            self.nodes.push(IrNode {
                type_: self.nodes[receiver].type_.clone(),
                opcode: ir::OpCode::Snapshot(thread.get_pc() as u64, SsaOrValue::Ssa(receiver)),
                operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                ssa_operands: vec![receiver],
            });
        }
        self.stack.push(self.nodes.len() - 1);
    }
}
