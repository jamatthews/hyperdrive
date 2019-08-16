use cranelift_codegen::ir::types::I64;
use std::mem::transmute;

use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;

use ir::*;
use ir::IrType::*;
use ir::OpCode::*;
use yarv_opcode::*;

pub fn compile(trace: Vec<IrNode>) -> fn() {
    let jit_builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    let mut module: Module<SimpleJITBackend> = Module::new(jit_builder);
    let mut codegen_context =  Context::new();

    module.make_signature();

    let func_id = module
        .declare_function("test", Linkage::Export, &codegen_context.func.signature)
        .expect("CraneLift error declaring function");

    let mut builder_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);

    translate_ir(trace, &mut builder);

    module
        .define_function(func_id, &mut codegen_context)
        .expect("CraneLift error defining function");

    module.finalize_definitions();
    let compiled_code = module.get_finalized_function(func_id);
    unsafe { transmute::<_, fn() >(compiled_code) }
}

fn translate_ir(trace: Vec<IrNode>, builder: &mut FunctionBuilder){
    let trace_block = builder.create_ebb();
    builder.switch_to_block(trace_block);

    let mut values = vec![];

    for node in &trace {
        match node.opcode {
            OpCode::Yarv(YarvOpCode::putobject_INT2FIX_1_) => {
                values.push(builder.ins().iconst(I64, 1 as i64));
            },
            OpCode::Yarv(YarvOpCode::opt_plus) => {
                let a = values[node.operand_1.expect("missing operand")];
                let b = values[node.operand_2.expect("missing operand")];
                values.push(builder.ins().iadd(a, b));
            },
            _=> {}
        };
    };

    builder.ins().return_(&[]);
}

#[cfg(test)]
mod tests {
    use super::*;

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
                opcode: Yarv(YarvOpCode::putobject_INT2FIX_1_),
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
                operand_1: Some(0),
                operand_2: Some(1),
            },
        ];
        let foo = compile(trace);
        foo();
    }
}
