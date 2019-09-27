use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, _instruction: Instruction, thread: Thread) {
    let value: Value = unsafe { *thread.get_sp().offset(-1) }.into();
    nodes.push(
        IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Guard(IrType::Yarv(value.type_())),
            operands: vec![],
            ssa_operands: vec![
                ssa_stack.pop().expect("ssa stack underflow in branch"),
            ],
        }
    );
    nodes.push(
        IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Snapshot(thread.get_pc() as u64 + 8),
            operands: vec![],
            ssa_operands: vec![],
        }
    );
}
