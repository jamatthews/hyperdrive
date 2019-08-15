use std::mem::transmute;

use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;


pub fn compile() -> fn() {
    let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    let mut module: Module<SimpleJITBackend> = Module::new(builder);
    let mut codegen_context =  Context::new();
    let mut builder_context = FunctionBuilderContext::new();

    module.make_signature();

    let func_id = module
        .declare_function("test", Linkage::Export, &codegen_context.func.signature)
        .expect("CraneLift error declaring function");

    let mut builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);

    println!("setup complete");

    let block = builder.create_ebb();
    builder.switch_to_block(block);
    builder.ins().return_(&[]);

    builder.seal_all_blocks();


    println!("defining function");
    module
        .define_function(func_id, &mut codegen_context)
        .expect("CraneLift error defining function");

    module.finalize_definitions();
        println!("defined function");
    let compiled_code = module.get_finalized_function(func_id);
    unsafe { transmute::<_, fn() >(compiled_code) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles() {
        let foo = compile();
        foo();
    }
}
