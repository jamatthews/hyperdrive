use yarv_instruction::YarvInstruction;
use ir::*;
use yarv_opcode::YarvOpCode;
use trace::IrNodes;
use vm_thread::VmThread;

pub fn record_instruction(nodes: &mut IrNodes, thread: VmThread) {
    let instruction = YarvInstruction::new(thread.get_pc());
    let opcode = instruction.opcode();

    match opcode {
        YarvOpCode::getlocal_WC_0 => {
            let offset = instruction.get_operand(0);
            let type_ =  match nodes.last() {
                Some(node) => node.type_.clone(),
                _ => IrType::Integer,
            };
            nodes.push(
                IrNode {
                    type_: type_,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![offset],
                }
            );
        },
        YarvOpCode::putobject_INT2FIX_1_ => {
            nodes.push(
                IrNode {
                    type_: IrType::Integer,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![],
                }
            );
        },
        YarvOpCode::opt_plus => {
            nodes.push(
                IrNode {
                    type_: IrType::Integer,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![],
                }
            );
        },
        YarvOpCode::setlocal_WC_0 => {
            let offset = instruction.get_operand(0);
            nodes.push(
                IrNode {
                    type_: nodes.last().expect("setlocal can't be first insn").type_.clone(),
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![offset],
                }
            );
        },
        YarvOpCode::putobject => {
            let object = instruction.get_operand(0);
            nodes.push(
                IrNode {
                    type_: IrType::Integer,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![object],
                }
            );
        },
        YarvOpCode::opt_lt => {
            nodes.push(
                IrNode {
                    type_: IrType::Boolean,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![],
                }
            );
        },
        YarvOpCode::branchif => {
            let target = instruction.get_operand(0);
            nodes.push(
                IrNode {
                    type_: IrType::None,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![target],
                }
            );
        },
        YarvOpCode::duparray => {
            let array = instruction.get_operand(0);
            nodes.push(
                IrNode {
                    type_: IrType::Array,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![array],
                }
            );
        },
        YarvOpCode::opt_send_without_block => {
            nodes.push(
                IrNode {
                    type_: IrType::Array,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![instruction.get_operand(0), instruction.get_operand(1)],
                }
            );
        },
        YarvOpCode::pop => {
            nodes.push(
                IrNode {
                    type_: IrType::Integer,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![],
                }
            );
        },
        YarvOpCode::putnil => {
            nodes.push(
                IrNode {
                    type_: IrType::Nil,
                    opcode: OpCode::Yarv(opcode),
                    operands: vec![],
                }
            );
        },
        _ => panic!("NYI: {:?}", opcode),
    }
}
