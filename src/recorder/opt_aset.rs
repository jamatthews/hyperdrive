use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, _instruction: Instruction, _thread: Thread) {
    let value = ssa_stack.pop().expect("ssa stack underflow in opt_aset");
    let key = ssa_stack.pop().expect("ssa stack underflow in opt_aset");
    let array = ssa_stack.pop().expect("ssa stack underflow in opt_aset");

    nodes.push(
        IrNode {
            type_: IrType::Internal(InternalType::Value),
            opcode: ir::OpCode::Yarv(OpCode::opt_aset),
            operands: vec![],
            ssa_operands: vec![array, key, value],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
