use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    instruction: Instruction,
    thread: Thread,
) {
    let raw_value = thread.get_self();
    let value: Value = raw_value.into();

    nodes.push(IrNode {
        type_: IrType::Yarv(value.type_()),
        opcode: ir::OpCode::Yarv(instruction.opcode()),
        operands: vec![],
        ssa_operands: vec![],
    });
    ssa_stack.push(nodes.len() - 1);
}
