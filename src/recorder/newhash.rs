use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    instruction: Instruction,
    _thread: Thread,
) {
    nodes.push(IrNode {
        type_: IrType::Yarv(ValueType::Hash),
        opcode: ir::OpCode::NewHash,
        operands: vec![],
        ssa_operands: vec![],
    });
    let hash = nodes.len() - 1;
    let count = instruction.get_operand(0);
    for _ in 0..(count / 2) {
        let value = ssa_stack.pop().expect("stack underflow recording newhash");
        let key = ssa_stack.pop().expect("stack underflow recording newhash");
        nodes.push(IrNode {
            type_: IrType::Yarv(ValueType::Hash),
            opcode: ir::OpCode::HashSet,
            operands: vec![],
            ssa_operands: vec![hash, key, value],
        });
    }
    ssa_stack.push(nodes.len() - 1);
}
