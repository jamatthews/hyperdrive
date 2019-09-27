use hyperdrive_ruby::rb_method_type_t_VM_METHOD_TYPE_CFUNC;
use ir;
use ir::*;
use trace::IrNodes;
use vm::*;
use vm::OpCode;

#[derive(Clone, Debug)]
pub struct TraceRecorder {
    pub nodes: IrNodes,
    stack: Vec<SsaRef>,
    pub anchor: u64,
    pub complete: bool,
}

impl TraceRecorder {
    pub fn new(anchor: u64) -> Self {
        Self {
            nodes: vec![],
            stack: vec![],
            anchor: anchor,
            complete: false,
        }
    }

    pub fn record_instruction(&mut self, thread: Thread) {
        let instruction = Instruction::new(thread.get_pc());
        let opcode = instruction.opcode();

        if !self.nodes.is_empty() && thread.get_pc() as u64 == self.anchor {
            self.nodes.push(
                IrNode {
                    type_: IrType::None,
                    opcode: ir::OpCode::Loop,
                    operands: vec![],
                    ssa_operands: vec![],
                }
            );
            self.complete = true;
            return
        }

        match opcode {
            OpCode::getlocal_WC_0 => {
                let offset = instruction.get_operand(0);
                let value: Value = thread.get_local(offset).into();
                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(value.type_()),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![offset],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::putobject_INT2FIX_1_ => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Fixnum),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_plus => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::I64),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_plus"),
                            self.stack.pop().expect("ssa stack underflow in opt_plus")
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::setlocal_WC_0 => {
                let offset = instruction.get_operand(0);
                let popped = self.stack.pop().expect("ssa stack underflow in setlocal");

                self.nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![offset],
                        ssa_operands: vec![
                            popped,
                        ]
                    }
                );
            },
            OpCode::putobject => {
                let raw_value = instruction.get_operand(0);
                let value: Value = raw_value.into();

                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(value.type_()),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![raw_value],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::putstring => {
                let raw_value = instruction.get_operand(0);
                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::RString),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![raw_value],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_eq => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_eq"),
                            self.stack.pop().expect("ssa stack underflow in opt_eq")
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_lt => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_lt"),
                            self.stack.pop().expect("ssa stack underflow in opt_lt")
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::branchif|OpCode::branchunless => {
                let value: Value = unsafe { *thread.get_sp().offset(-1) }.into();

                self.nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: ir::OpCode::Guard(IrType::Yarv(value.type_())),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in branch"),
                        ],
                    }
                );
                self.nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: ir::OpCode::Snapshot(thread.get_pc() as u64 + 8),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
            },
            OpCode::dup => {
                let popped = self.stack.pop().expect("ssa stack underflow in dup");
                self.nodes.push(
                    IrNode {
                        type_: self.nodes[popped].type_.clone(),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::duparray => {
                let array = instruction.get_operand(0);
                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Array),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![array],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_send_without_block => {
                let call_cache = CallCache::new(instruction.get_operand(1) as *const _);
                if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
                    self.nodes.push(
                        IrNode {
                            type_: IrType::Internal(InternalType::Value),
                            opcode: ir::OpCode::Yarv(opcode),
                            operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                            ssa_operands: vec![
                                self.stack.pop().expect("ssa stack underflow in opt_send_without_block"),
                            ],
                        }
                    );
                    self.stack.push(self.nodes.len() - 1);
                }
            },
            OpCode::pop => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in pop"),
                        ],
                    }
                );
            },
            OpCode::putnil => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Nil),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_empty_p => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_empty_p"),
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_not => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_not"),
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_aref => {
                self.nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Value),
                        opcode: ir::OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_aref"),
                        ],
                    }
                );
                self.stack.push(self.nodes.len() - 1);
            },
            OpCode::opt_ltlt => {
                let object = self.stack.pop().expect("ssa stack underflow in opt_ltlt");
                let receiver = self.stack.pop().expect("ssa stack underflow in opt_ltlt");

                match self.nodes[receiver].type_.clone() {
                    IrType::Yarv(ValueType::Array) => {
                        self.nodes.push(
                            IrNode {
                                type_: IrType::Yarv(ValueType::Array),
                                opcode: ir::OpCode::ArrayAppend,
                                operands: vec![],
                                ssa_operands: vec![
                                    object,
                                    receiver,
                                ],
                            }
                        );
                        self.stack.push(self.nodes.len() - 1);
                    }
                    x => panic!("NYI: opt_ltlt with: {:#?}", x),
                };
            },
            OpCode::putself|OpCode::leave => {},
            _ => panic!("NYI: {:?}", opcode),
        }
    }
}
