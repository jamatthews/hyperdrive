use compiler::*;
use ir::IrType::Integer;
use yarv_opcode::YarvOpCode;
use ir::*;

#[derive(Clone, Debug)]
pub struct Trace {
    pub nodes: Vec<IrNode>,
    pub anchor: u64,
    pub compiled_code: Option<fn() -> i64>,
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

    pub fn compile(&mut self){
        self.compiled_code = Some(compile(self.nodes.clone()));
    }
}
