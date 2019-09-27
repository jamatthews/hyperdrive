use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    let offset = instruction.get_operand(0);
    let popped = ssa_stack.pop().expect("ssa stack underflow in setlocal");

    nodes.push(
        IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![offset],
            ssa_operands: vec![
                popped,
            ]
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
