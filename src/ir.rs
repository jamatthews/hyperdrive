use yarv_types::ValueType;
use yarv_opcode::YarvOpCode;
use hyperdrive_ruby::VALUE;

pub type SsaRef = usize;

#[derive(Clone, Debug)]
pub struct IrNode {
    pub type_: IrType,
    pub opcode: OpCode,
    pub operands: Vec<VALUE>,
    pub ssa_operands: Vec<usize>,
}

#[derive(Clone, Debug)]
pub enum IrType {
    Yarv(ValueType),
    Internal(InternalType),
    None,
}

#[derive(Clone, Debug)]
pub enum InternalType {
    I64,
    Bool,
    Value,
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Yarv(YarvOpCode),
    Snapshot(u64),
    Guard(IrType),
    Loop,
    ArrayAppend,
}
