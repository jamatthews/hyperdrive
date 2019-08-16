use yarv_opcode::YarvOpCode;

pub struct IrNode {
    pub type_: IrType,
    pub opcode: OpCode,
    pub operand_1: Option<usize>,
    pub operand_2: Option<usize>
}

pub enum IrType {
    Integer
}

pub enum OpCode {
    Yarv(YarvOpCode),
}
