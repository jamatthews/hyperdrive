use super::*;

impl Recorder {
    pub fn record_opt_aset(&mut self, _thread: Thread, _instruction: Instruction) {
        let value = self.stack_pop();
        let key = self.stack_pop();
        let collection = self.stack_pop();

        let opcode = match &self.nodes[collection].type_() {
            IrType::Yarv(ValueType::Array) => ir::OpCode::ArraySet,
            IrType::Yarv(ValueType::Hash) => ir::OpCode::HashSet,
            x => panic!("aref not supported for {:#?}", x),
        };

        self.emit(IrNode::Basic {
            type_: self.nodes[value].type_(),
            opcode: opcode,
            operands: vec![],
            ssa_operands: vec![collection, key, value],
        });
        self.stack_push(self.nodes.len() - 1);
    }
}
