use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    let b = ssa_stack.pop().expect("ssa stack underflow in opt_lt");
    let a = ssa_stack.pop().expect("ssa stack underflow in opt_lt");
    nodes.push(
        IrNode {
            type_: IrType::Internal(InternalType::Bool),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![a,b],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
