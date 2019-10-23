use super::*;

pub fn record(
    nodes: &mut IrNodes,
    ssa_stack: &mut Vec<SsaRef>,
    _instruction: Instruction,
    _thread: Thread,
) {
    let value = ssa_stack.pop().expect("ssa stack underflow in opt_aset");
    let key = ssa_stack.pop().expect("ssa stack underflow in opt_aset");
    let collection = ssa_stack.pop().expect("ssa stack underflow in opt_aset");

    let opcode = match nodes[collection].type_.clone() {
        IrType::Yarv(ValueType::Array) => ir::OpCode::ArraySet,
        IrType::Yarv(ValueType::Hash) => ir::OpCode::HashSet,
        x => panic!("aref not supported for {:#?}", x),
    };

    nodes.push(IrNode {
        type_: nodes[value].type_.clone(),
        opcode: opcode,
        operands: vec![],
        ssa_operands: vec![collection, key, value],
    });
    ssa_stack.push(nodes.len() - 1);
}
