use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, instruction: Instruction, _thread: Thread) {
    let popped = ssa_stack.pop().expect("ssa stack underflow in dup");
    nodes.push(
        IrNode {
            type_: nodes[popped].type_.clone(),
            opcode: ir::OpCode::Yarv(instruction.opcode()),
            operands: vec![],
            ssa_operands: vec![],
        }
    );
    ssa_stack.push(nodes.len() - 1);
    ssa_stack.push(nodes.len() - 1);
}
