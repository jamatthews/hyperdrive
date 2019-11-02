use super::*;


impl Recorder {
    pub fn record_opt_aref(&mut self, _thread: Thread, _instruction: Instruction) {
        let key = self.stack.pop().expect("ssa stack underflow in opt_aref");
        let collection = self.stack.pop().expect("ssa stack underflow in opt_aref");

        let opcode = match self.nodes[collection].type_.clone() {
            IrType::Yarv(ValueType::Array) => ir::OpCode::ArrayGet,
            IrType::Yarv(ValueType::Hash) => ir::OpCode::HashGet,
            x => panic!(
                "aref not supported for {}[{}]\n{:#?}",
                collection, key, self.nodes
            ),
        };

        self.nodes.push(IrNode {
            type_: IrType::Internal(InternalType::Value),
            opcode: opcode,
            operands: vec![],
            ssa_operands: vec![collection, key],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
