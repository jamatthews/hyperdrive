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
use vm::OpCode;
use vm::*;

#[derive(Clone, Debug)]
pub struct Recorder {
    pub nodes: IrNodes,
    pub anchor: u64,
    stack: HashMap<isize, SsaRef>,
    sp_base: *const u64,
    sp_offset: isize,
}

impl Recorder {
    pub fn new(thread: Thread) -> Self {
        Self {
            nodes: vec![],
            stack: HashMap::new(),
            anchor: thread.get_pc() as u64,
            sp_base: thread.get_sp(),
            sp_offset: 0,
        }
    }

    fn stack_pop(&mut self) -> SsaRef {
        self.sp_offset -= 1;
        let ret = match self.stack.remove(&self.sp_offset) {
            Some(ssa_ref) => ssa_ref,
            None => {
                let value: Value = unsafe { *self.sp_base.offset(-1 * self.sp_offset) }.into();
                self.nodes.push(IrNode {
                    type_: IrType::Yarv(value.type_()),
                    opcode: ir::OpCode::StackLoad,
                    operands: vec![],
                    ssa_operands: vec![],
                });
                self.nodes.len() - 1
            }
        };
        ret
    }

    fn stack_push(&mut self, ssa_ref: SsaRef) {
        self.stack.insert(self.sp_offset, ssa_ref);
        self.sp_offset += 1;
    }

    pub fn record_instruction(&mut self, thread: Thread) -> Result<bool, String> {
        let instruction = Instruction::new(thread.get_pc());
        let opcode = instruction.opcode();

        if !self.nodes.is_empty() && thread.get_pc() as u64 == self.anchor {
            self.nodes.push(IrNode {
                type_: IrType::None,
                opcode: ir::OpCode::Loop,
                operands: vec![],
                ssa_operands: vec![],
            });
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
            OpCode::opt_send_without_block => {
                self.record_opt_send_without_block(thread, instruction)
            }
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

            OpCode::leave => {}
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
}
