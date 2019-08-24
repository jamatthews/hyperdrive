use yarv_opcode::YarvOpCode;

#[derive(Clone, Debug)]
pub struct IrNode {
    pub pc: u64,
    pub type_: IrType,
    pub opcode: OpCode,
    pub operand_1: Option<usize>,
    pub operand_2: Option<usize>
}

#[derive(Clone, Debug)]
pub enum IrType {
    Integer
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Yarv(YarvOpCode),
}
