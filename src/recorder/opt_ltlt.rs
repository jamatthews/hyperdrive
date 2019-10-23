use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    _instruction: Instruction,
    _thread: Thread,
) {
    let object = ssa_stack.pop().expect("ssa stack underflow in opt_ltlt");
    let receiver = ssa_stack.pop().expect("ssa stack underflow in opt_ltlt");

    match nodes[receiver].type_.clone() {
        IrType::Yarv(ValueType::Array) => {
            nodes.push(IrNode {
                type_: IrType::Yarv(ValueType::Array),
                opcode: ir::OpCode::ArrayAppend,
                operands: vec![],
                ssa_operands: vec![receiver, object],
            });
            ssa_stack.push(nodes.len() - 1);
        }
        x => panic!("NYI: opt_ltlt with: {:#?}", x),
    };
}
