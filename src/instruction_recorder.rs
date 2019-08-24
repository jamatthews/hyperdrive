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
            nodes.push(
                IrNode {
                    type_: IrType::Integer,
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
                    type_: IrType::None,
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
        _ => panic!("NYI: {:?}", opcode),
    }
}
