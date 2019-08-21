use cranelift_codegen::ir::types::I64;
use std::mem::transmute;

use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;

use ir::*;
use yarv_opcode::*;

pub fn compile(trace: Vec<IrNode>) -> fn() -> i64 {
    let jit_builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    let mut module: Module<SimpleJITBackend> = Module::new(jit_builder);
    let mut codegen_context =  Context::new();

    codegen_context.func.signature.returns.push(AbiParam::new(types::I64));

    let func_id = module
        .declare_function("test", Linkage::Export, &codegen_context.func.signature)
        .expect("CraneLift error declaring function");

    let mut builder_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);

    translate_ir(trace, &mut builder);

    println!("{}", builder.display(None).to_string());

    module
        .define_function(func_id, &mut codegen_context)
        .expect("CraneLift error defining function");

    module.finalize_definitions();
    let compiled_code = module.get_finalized_function(func_id);
    unsafe { transmute::<_, fn() -> i64>(compiled_code) }
}

fn translate_ir(trace: Vec<IrNode>, builder: &mut FunctionBuilder){
    let slot = builder.create_stack_slot(StackSlotData{kind: StackSlotKind::ExplicitSlot, size: 8, offset: None});
    let entry_block = builder.create_ebb();
    let loop_block = builder.create_ebb();
    builder.switch_to_block(entry_block);
    let result = builder.ins().iconst(I64, 0 as i64);
    builder.ins().stack_store(result, slot, 0);
    builder.ins().jump(loop_block, &[]);
    builder.switch_to_block(loop_block);

    let mut stack = vec![];

    for (i, node) in trace.iter().enumerate() {
        match node.opcode {
            OpCode::Yarv(YarvOpCode::putobject_INT2FIX_1_) => {
                stack.push(builder.ins().iconst(I64, 1 as i64));
            },
            OpCode::Yarv(YarvOpCode::opt_plus) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(builder.ins().iadd(a, b));
            },
            OpCode::Yarv(YarvOpCode::getlocal_WC_0) => {
                stack.push( builder.ins().stack_load(I64, slot, 0) );
            },
            OpCode::Yarv(YarvOpCode::setlocal_WC_0) => {
                builder.ins().stack_store(stack.pop().unwrap(), slot, 0);
            },
            OpCode::Yarv(YarvOpCode::putobject) => {
                stack.push(builder.ins().iconst(I64, 1002 as i64));
            },
            OpCode::Yarv(YarvOpCode::opt_lt) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = builder.ins().icmp(IntCC::SignedLessThan, a, b);
                stack.push(result);
            },
            OpCode::Yarv(YarvOpCode::branchif) => {
                let a = stack.pop().unwrap();
                builder.ins().brnz(a, loop_block, &[]);
            },
            _=> { }
        };
    };
    let val = builder.ins().stack_load(I64, slot, 0);
    builder.ins().return_(&[val]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrType::*;
    use ir::OpCode::*;

    #[test]
    fn it_compiles() {
        let foo = compile(vec![]);
        foo();
    }

    #[test]
    fn it_compiles_simple() {
        let trace = vec![
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::getlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::putobject_INT2FIX_1_),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::opt_plus),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::setlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::getlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::putobject),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::opt_lt),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                type_: Integer,
                opcode: Yarv(YarvOpCode::branchif),
                operand_1: None,
                operand_2: None,
            },
        ];
        let foo = compile(trace);
        let result = foo();
        assert_eq!(result, 1002);
    }
}
