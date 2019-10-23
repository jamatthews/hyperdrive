use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    instruction: Instruction,
    _thread: Thread,
) {
    nodes.push(IrNode {
        type_: IrType::Yarv(ValueType::Array),
        opcode: ir::OpCode::NewArray,
        operands: vec![],
        ssa_operands: vec![],
    });
    let array = nodes.len() - 1;
    let count = instruction.get_operand(0);
    for _ in 0..count {
        let object = ssa_stack.pop().expect("stack underflow recording arraynew");
        nodes.push(IrNode {
            type_: IrType::Yarv(ValueType::Array),
            opcode: ir::OpCode::ArrayAppend,
            operands: vec![],
            ssa_operands: vec![array, object],
        });
    }
    ssa_stack.push(nodes.len() - 1);
}
