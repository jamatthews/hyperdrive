mod branch;
mod comparison;
mod dup;
mod duparray;
mod getlocal_wc_0;
mod newarray;
mod newhash;
mod opt_aref;
mod opt_aset;
mod opt_empty_p;
mod opt_ltlt;
mod opt_not;
mod opt_plus;
mod opt_send_without_block;
mod putnil;
mod putobject;
mod putobject_fix;
mod putself;
mod putstring;
mod setlocal_wc_0;

use ir;
use ir::*;
use std::collections::HashMap;
use trace::IrNodes;
use vm;
use vm::OpCode;
use vm::*;

#[derive(Clone, Debug)]
pub struct Recorder {
    pub nodes: IrNodes,
    pub anchor: u64,
    stack: HashMap<isize, SsaRef>,
    base_ep: *const u64,
    sp: isize,
    ep: isize,
    call_stack: Vec<SsaRef>,
}

impl Recorder {
    pub fn new(thread: Thread) -> Self {
        //the actual object of self is loaded at execution time, but we need to load it here to add a type
        let raw_value = thread.get_self();
        let value: Value = raw_value.into();

        let nodes = vec![ IrNode {
                type_: IrType::Yarv(value.type_()),
                opcode: ir::OpCode::LoadSelf,
                operands: vec![],
                ssa_operands: vec![],
            }];

        Self {
            nodes: nodes,
            stack: HashMap::new(),
            anchor: thread.get_pc() as u64,
            base_ep: thread.get_ep(),
            sp: (thread.get_sp() as u64 - thread.get_ep() as u64) as isize, //keep SP as relative so we can restore relative to EP
            ep: 0,
            call_stack: vec![0],
        }
    }

    fn stack_n(&self, offset: usize) -> SsaRef {
        *self.stack.get(&(self.sp - 8 - (offset * 8) as isize)).expect("stack underflow in n")
    }

    fn stack_peek(&mut self) -> SsaRef {
        *self.stack.get(&(self.sp - 8)).expect("stack underflow in peek")
    }

    fn stack_pop(&mut self) -> SsaRef {
        self.sp -= 8;
        let ret = match self.stack.remove(&self.sp) {
            Some(ssa_ref) => ssa_ref,
            None => {
                let value: Value = unsafe { *self.base_ep.offset(self.sp) }.into();
                self.nodes.push(IrNode {
                    type_: IrType::Yarv(value.type_()),
                    opcode: ir::OpCode::StackLoad(self.sp),
                    operands: vec![],
                    ssa_operands: vec![],
                });
                self.nodes.len() - 1
            }
        };
        ret
    }

    fn stack_push(&mut self, ssa_ref: SsaRef) {
        self.stack.insert(self.sp, ssa_ref);
        self.sp += 8;
    }

    pub fn record_instruction(&mut self, thread: Thread) -> Result<bool, String> {
        self.ep = (thread.get_ep() as u64 - self.base_ep as u64) as isize;
        self.sp = (thread.get_sp() as u64 - thread.get_ep() as u64) as isize;
        let instruction = Instruction::new(thread.get_pc());
        let opcode = instruction.opcode();

        self.ep = (thread.get_ep() as u64 - self.base_ep as u64) as isize;
        self.sp = (thread.get_sp() as u64 - thread.get_ep() as u64) as isize;

        if !self.nodes.is_empty() && thread.get_pc() as u64 == self.anchor {
            let snapshot = self.snapshot(thread);
            self.nodes.push(IrNode {
                type_: IrType::None,
                opcode: ir::OpCode::Snapshot(snapshot),
                operands: vec![],
                ssa_operands: vec![],
            });
            self.peel();
            return Ok(true);
        }

        match opcode {
            OpCode::branchif | OpCode::branchunless => self.record_branch(thread, instruction),
            OpCode::opt_eq | OpCode::opt_lt => self.record_comparison(thread, instruction),
            OpCode::dup => self.record_dup(thread, instruction),
            OpCode::duparray => self.record_duparray(thread, instruction),
            OpCode::getlocal_WC_0 => self.record_getlocal(thread, instruction),
            OpCode::newarray => self.record_newarray(thread, instruction),
            OpCode::newhash => self.record_newhash(thread, instruction),
            OpCode::opt_aref => self.record_opt_aref(thread, instruction),
            OpCode::opt_aset => self.record_opt_aset(thread, instruction),
            OpCode::opt_empty_p => self.record_opt_empty_p(thread, instruction),
            OpCode::opt_ltlt => self.record_opt_ltlt(thread, instruction),
            OpCode::opt_not => self.record_opt_not(thread, instruction),
            OpCode::opt_plus => self.record_opt_plus(thread, instruction),
            OpCode::opt_send_without_block => self.record_opt_send_without_block(thread, instruction),
            OpCode::pop => {
                self.stack_pop();
            }
            OpCode::putnil => self.record_putnil(thread, instruction),
            OpCode::putobject => self.record_putobject(thread, instruction),
            OpCode::putobject_INT2FIX_0_ | OpCode::putobject_INT2FIX_1_ => {
                self.record_putobject_fix(thread, instruction)
            }
            OpCode::putself => self.record_putself(thread, instruction),
            OpCode::putstring => self.record_putstring(thread, instruction),
            OpCode::setlocal_WC_0 => self.record_setlocal(thread, instruction),
            OpCode::leave => {
                self.sp = self.sp - 2;
                self.call_stack.pop();
            },
            OpCode::jump => {}
            _ => return Err(format!("NYI: {:?}", opcode)),
        }

        Ok(false)
    }

    fn snapshot(&mut self, thread: Thread) -> Snapshot {
        Snapshot {
            pc: thread.get_pc() as u64,
            sp: thread.get_sp() as u64,
            self_: SsaOrValue::Value(thread.get_self()),
            stack_map: self.stack.clone(),
        }
    }

    fn peel(&mut self) {
        let peeled = self.nodes.clone();
        let base_snap = match &self.nodes.last().unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing base snapshot"),
        };
        self.nodes.push(IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Loop,
            operands: vec![],
            ssa_operands: vec![],
        });
        let offset = peeled.len() + 1;
        for node in &peeled {
            let (opcode, type_) = match &node.opcode {
                ir::OpCode::Guard(type_, snap) => (
                    ir::OpCode::Guard(type_.clone(), self.copy_snapshot(snap, offset)),
                    node.type_.clone(),
                ),
                ir::OpCode::Snapshot(snap) => (
                    ir::OpCode::Snapshot(self.copy_snapshot(snap, offset)),
                    node.type_.clone(),
                ),
                ir::OpCode::StackLoad(offset) => {
                    let ssa_ref = *base_snap
                        .get(&offset)
                        .expect(&format!("missing entry in stackmap for: {}", offset));
                    (ir::OpCode::Pass(ssa_ref), self.nodes[ssa_ref].type_.clone())
                }
                op => (op.clone(), node.type_.clone()),
            };
            self.nodes.push(IrNode {
                type_: type_,
                opcode: opcode,
                operands: node.operands.clone(),
                ssa_operands: node.ssa_operands.iter().map(|op| *op + peeled.len() + 1).collect(),
            });
        }
        self.phi(peeled.len() - 1);
    }

    fn copy_snapshot(&self, snap: &Snapshot, bias: usize) -> Snapshot {
        let mut updated = HashMap::new();
        for (offset, ssa_ref) in snap.stack_map.iter() {
            updated.insert(offset.clone(), ssa_ref + bias);
        }
        Snapshot {
            pc: snap.pc,
            sp: snap.sp,
            self_: snap.self_.clone(),
            stack_map: updated,
        }
    }

    pub fn phi(&mut self, idx: usize) {
        let after = match &self.nodes.last().unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing after snapshot"),
        };
        let before = match &self.nodes.get(idx).unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing before snapshot"),
        };

        for (slot, ssa_ref) in after.iter() {
            if before.get(slot) != Some(ssa_ref) {
                self.nodes.push(IrNode {
                    type_: IrType::None,
                    opcode: ir::OpCode::Phi,
                    operands: vec![],
                    ssa_operands: vec![*before.get(slot).unwrap(), *ssa_ref],
                });
            }
        }
    }
}
