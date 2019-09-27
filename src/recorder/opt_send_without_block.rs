use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    let call_cache = CallCache::new(instruction.get_operand(1) as *const _);
    if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
        nodes.push(
            IrNode {
                type_: IrType::Internal(InternalType::Value),
                opcode: ir::OpCode::Yarv(instruction.opcode()),
                operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                ssa_operands: vec![
                    ssa_stack.pop().expect("ssa ssa_stack underflow in opt_send_without_block"),
                ],
            }
        );
        ssa_stack.push(nodes.len() - 1);
    }
}
