use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    let raw_value = instruction.get_operand(0);

    nodes.push(
        IrNode {
            type_: IrType::Yarv(ValueType::RString),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![raw_value],
            ssa_operands: vec![],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
