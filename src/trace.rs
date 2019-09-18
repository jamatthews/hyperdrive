use ir::*;
use hyperdrive_ruby::VALUE;
use std::mem::transmute;
use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_module::*;
use cranelift_simplejit::*;

use trace_compiler::TraceCompiler;


pub type IrNodes = Vec<IrNode>;

#[derive(Clone, Debug)]
pub struct Trace {
    pub nodes: IrNodes,
    pub anchor: u64,
    pub compiled_code: Option<fn(*const VALUE) -> i64>,
}

impl Trace {
    pub fn new(pc: u64, nodes: IrNodes) -> Self {
        Trace {
            anchor: pc,
            nodes: nodes,
            compiled_code: None,
        }
    }

    pub fn compile(&mut self, module: &mut Module<SimpleJITBackend>){
        let mut codegen_context = Context::new();
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));

        let func_id = module
            .declare_function(&self.anchor.to_string(), Linkage::Export, &codegen_context.func.signature)
            .expect("CraneLift error declaring function");

        {
            let mut builder_context = FunctionBuilderContext::new();
            let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
            let mut compiler = TraceCompiler::new(module, builder);
            compiler.compile(self.nodes.clone());
        }

        module
            .define_function(func_id, &mut codegen_context)
            .expect("CraneLift error defining function");

        module.finalize_definitions();
            let compiled_code = module.get_finalized_function(func_id);

        self.compiled_code = Some(unsafe { transmute::<_, fn(*const VALUE) -> i64>(compiled_code) });
    }

    pub fn preview(&mut self, module: &mut Module<SimpleJITBackend>) -> String {
        let mut codegen_context = Context::new();
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        let _func_id = module
            .declare_function("test", Linkage::Export, &codegen_context.func.signature)
            .expect("CraneLift error declaring function");

        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = TraceCompiler::new(module, builder);
        compiler.compile(self.nodes.clone());
        compiler.preview().unwrap()
    }
}
