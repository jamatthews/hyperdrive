pub struct IrNode {
    type_: IrType,
    opcode: OpCode,
    operand_1: Option<Operand>,
    operand_2: Option<Operand>
}

pub enum IrType {
    Integer
}

pub enum OpCode {
    StackPush,
    Plus
}

pub enum Operand {
    Constant,
    SsaReference,
}
