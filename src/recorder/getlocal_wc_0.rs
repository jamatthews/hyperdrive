use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, thread: Thread) {

    let offset = instruction.get_operand(0);
    let value: Value = thread.get_local(offset).into();
    nodes.push(
        IrNode {
            type_: IrType::Yarv(value.type_()),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![offset],
            ssa_operands: vec![],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
