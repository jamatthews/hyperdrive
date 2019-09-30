use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, _instruction: Instruction, _thread: Thread) {
    let value = ssa_stack.pop().expect("ssa stack underflow in opt_aref");
    let array = ssa_stack.pop().expect("ssa stack underflow in opt_aref");
    nodes.push(
        IrNode {
            type_: IrType::Internal(InternalType::Value),
            opcode: ir::OpCode::ArrayRef,
            operands: vec![],
            ssa_operands: vec![array, value],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
