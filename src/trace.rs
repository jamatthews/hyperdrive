use opcode::OpCode;

pub struct Trace {
    pub opcodes: Vec<OpCode>,
    pub anchor: u64,
}

impl Trace {
    pub fn add_opcode(&mut self, opcode: OpCode){
        self.opcodes.push(opcode);
    }
}
