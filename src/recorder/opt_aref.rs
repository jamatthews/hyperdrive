use super::*;

impl Recorder {
    pub fn record_opt_aref(&mut self, _thread: Thread, _instruction: Instruction) {
        let key = self.stack_pop();
        let collection = self.stack_pop();

        let opcode = match self.nodes[collection].type_.clone() {
            IrType::Yarv(ValueType::Array) => ir::OpCode::ArrayGet,
            IrType::Yarv(ValueType::Hash) => ir::OpCode::HashGet,
            _ => panic!(),
        };

        self.nodes.push(IrNode {
            type_: IrType::Internal(InternalType::Value),
            opcode: opcode,
            operands: vec![],
            ssa_operands: vec![collection, key],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
