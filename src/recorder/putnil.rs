use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    instruction: Instruction,
    _thread: Thread,
) {
    nodes.push(IrNode {
        type_: IrType::Yarv(ValueType::Nil),
        opcode: ir::OpCode::Yarv(instruction.opcode()),
        operands: vec![],
        ssa_operands: vec![],
    });
    ssa_stack.push(nodes.len() - 1);
}
