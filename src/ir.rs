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

#[derive(Clone, Debug, PartialEq)]
pub enum IrType {
    Yarv(ValueType),
    Internal(InternalType),
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InternalType {
    I64,
    Bool,
    Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub pc: u64,
    pub sp: u64,
    pub self_: SsaOrValue,
    pub stack_map: HashMap<isize, SsaRef>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    Loop, //marks the end of the prelude and start of the loop body
    Phi,
    Yarv(vm::OpCode),
    Snapshot(Snapshot),
    StackLoad,
    Guard(IrType, Snapshot),
    ArrayAppend,
    ArrayGet,
    NewArray,
    ArraySet,
    NewHash,
    HashSet,
    HashGet,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SsaOrValue {
    Ssa(usize),
    Value(VALUE),
}
