use super::*;

impl Recorder {
    pub fn record_opt_aset(&mut self, _thread: Thread, _instruction: Instruction) {
        let value = self.stack.pop().expect("ssa stack underflow in opt_aset");
        let key = self.stack.pop().expect("ssa stack underflow in opt_aset");
        let collection = self.stack.pop().expect("ssa stack underflow in opt_aset");

        let opcode = match self.nodes[collection].type_.clone() {
            IrType::Yarv(ValueType::Array) => ir::OpCode::ArraySet,
            IrType::Yarv(ValueType::Hash) => ir::OpCode::HashSet,
            x => panic!("aref not supported for {:#?}", x),
        };

        self.nodes.push(IrNode {
            type_: self.nodes[value].type_.clone(),
            opcode: opcode,
            operands: vec![],
            ssa_operands: vec![collection, key, value],
        });
        self.stack.push(self.nodes.len() - 1);
    }
}
