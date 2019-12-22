use super::*;

use hyperdrive_ruby::rb_method_type_t_VM_METHOD_TYPE_CFUNC;

impl Recorder {
    pub fn record_opt_send_without_block(&mut self, _thread: Thread, instruction: Instruction) {
        let call_info = CallInfo::new(instruction.get_operand(0) as *const _);
        let call_cache = CallCache::new(instruction.get_operand(1) as *const _);

        if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
            let receiver = self.stack_pop();
            self.nodes.push(IrNode::Basic {
                type_: IrType::Internal(InternalType::Value),
                opcode: ir::OpCode::Yarv(instruction.opcode()),
                operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                ssa_operands: vec![receiver],
            });
            self.stack_push(self.nodes.len() - 1);
        } else {
            let receiver = self.stack_n(call_info.get_orig_argc() as usize);
            self.call_stack.push(Frame { self_: receiver, sp: self.sp });
        }
    }
}
