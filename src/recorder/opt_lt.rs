use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    nodes.push(
        IrNode {
            type_: IrType::Internal(InternalType::Bool),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![
                ssa_stack.pop().expect("ssa stack underflow in opt_lt"),
                ssa_stack.pop().expect("ssa stack underflow in opt_lt")
            ],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
