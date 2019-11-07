use hyperdrive_ruby::VALUE;
use std::collections::HashMap;
use vm;
use vm::*;

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
pub struct Snapshot {
    pub pc: u64,
    pub sp: u64,
    pub self_: SsaOrValue,
    pub stack_map: HashMap<isize, SsaRef>,
}

#[derive(Clone, Debug)]
pub enum OpCode {
    Yarv(vm::OpCode),
    Snapshot(Snapshot),
    StackLoad,
    Guard(IrType, Snapshot),
    Loop,
    ArrayAppend,
    ArrayGet,
    NewArray,
    ArraySet,
    NewHash,
    HashSet,
    HashGet,
}

#[derive(Clone, Debug)]
pub enum SsaOrValue {
    Ssa(usize),
    Value(VALUE),
}
