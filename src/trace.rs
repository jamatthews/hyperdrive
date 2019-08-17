use ir::IrType::Integer;
use yarv_opcode::YarvOpCode;
use ir::*;

pub struct Trace {
    pub nodes: Vec<IrNode>,
    pub anchor: u64,
}

impl Trace {
    pub fn add_node(&mut self, opcode: YarvOpCode){
        let node = IrNode {
            type_: Integer,
            opcode: OpCode::Yarv(opcode),
            operand_1: None,
            operand_2: None,
        };
        self.nodes.push(node);
    }
}
