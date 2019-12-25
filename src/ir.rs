use hyperdrive_ruby::VALUE;
use std::collections::HashMap;
use vm;
use vm::*;

pub type SsaRef = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum IrNode {
    Basic {
        type_: IrType,
        opcode: OpCode,
        operands: Vec<VALUE>,
        ssa_operands: Vec<SsaRef>,
    },
    Constant {
        type_: IrType,
        reference: VALUE,
    },
    Guard {
        type_: IrType,
        ssa_ref: SsaRef,
        snap: Snapshot,
    },
    Snapshot {
        snap: Snapshot,
    },
}

impl IrNode {
    pub fn type_(&self) -> IrType {
        match self {
            IrNode::Basic { type_, .. } => type_.clone(),
            IrNode::Constant { type_, .. } => type_.clone(),
            IrNode::Guard { type_, .. } => type_.clone(),
            IrNode::Snapshot { .. } => IrType::None,
        }
    }

    pub fn opcode(&self) -> OpCode {
        match self {
            IrNode::Basic { opcode, .. } => opcode.clone(),
            _ => OpCode::None,
        }
    }

    pub fn operands(&self) -> Vec<VALUE> {
        match self {
            IrNode::Basic { operands, .. } => operands.clone(),
            _ => vec![],
        }
    }

    pub fn ssa_operands(&self) -> Vec<SsaRef> {
        match self {
            IrNode::Basic { ssa_operands, .. } => ssa_operands.clone(),
            IrNode::Constant { .. } => vec![],
            IrNode::Guard { ssa_ref, .. } => vec![*ssa_ref],
            IrNode::Snapshot { .. } => vec![],
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            IrNode::Constant { .. } => true,
            _ => false,
        }
    }
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
pub struct Frame {
    pub self_: SsaRef,
    pub pc: u64,
    pub sp: isize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub stack_map: HashMap<isize, SsaRef>,
    pub call_stack: Vec<Frame>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpCode {
    None,
    Loop, //marks the end of the prelude and start of the loop body
    Phi,
    Pass(SsaRef),
    LoadSelf,
    Yarv(vm::OpCode),
    StackLoad(isize),
    ArrayAppend,
    ArrayGet,
    NewArray,
    ArraySet,
    NewHash,
    HashSet,
    HashGet,
}
