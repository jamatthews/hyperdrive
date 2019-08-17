use yarv_opcode::YarvOpCode;

#[derive(Debug)]
pub struct IrNode {
    pub type_: IrType,
    pub opcode: OpCode,
    pub operand_1: Option<usize>,
    pub operand_2: Option<usize>
}

#[derive(Debug)]
pub enum IrType {
    Integer
}

#[derive(Debug)]
pub enum OpCode {
    Yarv(YarvOpCode),
}
