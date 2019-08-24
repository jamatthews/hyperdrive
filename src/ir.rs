use yarv_opcode::YarvOpCode;
use hyperdrive_ruby::VALUE;

#[derive(Clone, Debug)]
pub struct IrNode {
    pub type_: IrType,
    pub opcode: OpCode,
    pub operands: Vec<VALUE>,
}

#[derive(Clone, Debug)]
pub enum IrType {
    Integer,
    Boolean,
    None,
    Snapshot,
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Yarv(YarvOpCode),
    Snapshot(u64),
}
