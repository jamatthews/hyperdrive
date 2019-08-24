use hyperdrive_ruby::rb_vm_insn_addr2insn;
use yarv_opcode::YarvOpCode;
use hyperdrive_ruby::VALUE;

#[derive(Clone, Debug)]
pub struct YarvInstruction {
    pub instruction: *const VALUE,
}

impl YarvInstruction {
    pub fn new(pc: *const VALUE) -> Self {
        Self {
            instruction: pc
        }
    }

    pub fn opcode(&self) -> YarvOpCode {
        let raw_opcode: i32 = unsafe { rb_vm_insn_addr2insn(*self.instruction as *const _) };
        unsafe { std::mem::transmute(raw_opcode) }
    }

    pub fn get_operand(&self, offset: u64) -> VALUE {
        unsafe { *self.instruction.offset((offset + 1) as isize) }
    }
}
