use yarv_types::Value;
use hyperdrive_ruby::rb_method_type_t_VM_METHOD_TYPE_CFUNC;
use yarv_instruction::YarvInstruction;
use ir::*;
use yarv_opcode::YarvOpCode;
use yarv_types::ValueType;
use trace::IrNodes;
use vm_call_cache::VmCallCache;
use vm_thread::VmThread;

#[derive(Clone, Debug)]
pub struct InstructionRecorder {
    pub stack: Vec<SsaRef>
}

impl InstructionRecorder {
    pub fn record_instruction(&mut self, nodes: &mut IrNodes, thread: VmThread) {
        let instruction = YarvInstruction::new(thread.get_pc());
        let opcode = instruction.opcode();

        match opcode {
            YarvOpCode::getlocal_WC_0 => {
                let offset = instruction.get_operand(0);
                let value: Value = thread.get_local(offset).into();
                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(value.type_()),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![offset],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::putobject_INT2FIX_1_ => {
                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Fixnum),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_plus => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::I64),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_plus"),
                            self.stack.pop().expect("ssa stack underflow in opt_plus")
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::setlocal_WC_0 => {
                let offset = instruction.get_operand(0);
                let popped = self.stack.pop().expect("ssa stack underflow in setlocal");

                nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![offset],
                        ssa_operands: vec![
                            popped,
                        ]
                    }
                );
            },
            YarvOpCode::putobject => {
                let raw_value = instruction.get_operand(0);
                let value: Value = raw_value.into();

                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(value.type_()),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![raw_value],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::putstring => {
                let raw_value = instruction.get_operand(0);
                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::RString),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![raw_value],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_eq => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_eq"),
                            self.stack.pop().expect("ssa stack underflow in opt_eq")
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_lt => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_lt"),
                            self.stack.pop().expect("ssa stack underflow in opt_lt")
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::branchif => {
                let target = instruction.get_operand(0);
                nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![target],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in branchif"),
                        ],
                    }
                );
            },
            YarvOpCode::branchunless => {
                let target = instruction.get_operand(0);
                nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![target],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in branchunless"),
                        ],
                    }
                );
            },
            YarvOpCode::dup => {
                let popped = self.stack.pop().expect("ssa stack underflow in dup");
                nodes.push(
                    IrNode {
                        type_: nodes[popped].type_.clone(),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::duparray => {
                let array = instruction.get_operand(0);
                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Array),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![array],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_send_without_block => {
                let call_cache = VmCallCache::new(instruction.get_operand(1) as *const _);
                if call_cache.get_type() == rb_method_type_t_VM_METHOD_TYPE_CFUNC {
                    nodes.push(
                        IrNode {
                            type_: IrType::Internal(InternalType::Value),
                            opcode: OpCode::Yarv(opcode),
                            operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                            ssa_operands: vec![
                                self.stack.pop().expect("ssa stack underflow in opt_send_without_block"),
                            ],
                        }
                    );
                    self.stack.push(nodes.len() - 1);
                }
            },
            YarvOpCode::pop => {
                nodes.push(
                    IrNode {
                        type_: IrType::None,
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in pop"),
                        ],
                    }
                );
            },
            YarvOpCode::putnil => {
                nodes.push(
                    IrNode {
                        type_: IrType::Yarv(ValueType::Nil),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_empty_p => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_empty_p"),
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_not => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Bool),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_not"),
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::opt_aref => {
                nodes.push(
                    IrNode {
                        type_: IrType::Internal(InternalType::Value),
                        opcode: OpCode::Yarv(opcode),
                        operands: vec![],
                        ssa_operands: vec![
                            self.stack.pop().expect("ssa stack underflow in opt_aref"),
                        ],
                    }
                );
                self.stack.push(nodes.len() - 1);
            },
            YarvOpCode::putself|YarvOpCode::leave => {},
            _ => panic!("NYI: {:?}", opcode),
        }
    }
}
