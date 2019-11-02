use super::*;

impl Recorder {
    pub fn record_opt_ltlt(&mut self, _thread: Thread, _instruction: Instruction) {
        let object = self.stack.pop().expect("ssa stack underflow in opt_ltlt");
        let receiver = self.stack.pop().expect("ssa stack underflow in opt_ltlt");

        match self.nodes[receiver].type_.clone() {
            IrType::Yarv(ValueType::Array) => {
                self.nodes.push(IrNode {
                    type_: IrType::Yarv(ValueType::Array),
                    opcode: ir::OpCode::ArrayAppend,
                    operands: vec![],
                    ssa_operands: vec![receiver, object],
                });
                self.stack.push(self.nodes.len() - 1);
            }
            x => panic!("NYI: opt_ltlt with: {:#?}", x),
        };
    }
}
