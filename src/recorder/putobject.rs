use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    instruction: Instruction,
    _thread: Thread,
) {
    let raw_value = instruction.get_operand(0);
    let value: Value = raw_value.into();

    let type_ = if value.type_() == ValueType::Fixnum {
        IrType::Internal(InternalType::I64)
    } else {
        IrType::Yarv(value.type_())
    };

    nodes.push(IrNode {
        type_: type_,
        opcode: ir::OpCode::Yarv(instruction.opcode()),
        operands: vec![raw_value],
        ssa_operands: vec![],
    });
    ssa_stack.push(nodes.len() - 1);
}
