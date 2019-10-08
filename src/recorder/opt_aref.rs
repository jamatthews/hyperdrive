use super::*;

pub fn record(nodes: &mut IrNodes, ssa_stack: &mut Vec<SsaRef>, _instruction: Instruction, _thread: Thread) {
    let key = ssa_stack.pop().expect("ssa stack underflow in opt_aref");
    let collection = ssa_stack.pop().expect("ssa stack underflow in opt_aref");

    let opcode = match nodes[collection].type_.clone() {
        IrType::Yarv(ValueType::Array) => ir::OpCode::ArrayGet,
        IrType::Yarv(ValueType::Hash) => ir::OpCode::HashGet,
        x => panic!("aref not supported for {}[{}]\n{:#?}", collection, key, nodes),
    };

    nodes.push(
        IrNode {
            type_: IrType::Internal(InternalType::Value),
            opcode: opcode,
            operands: vec![],
            ssa_operands: vec![collection, key],
        }
    );
    ssa_stack.push(nodes.len() - 1);
}
