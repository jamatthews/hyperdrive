use hyperdrive_ruby::rb_str_strlen;
use ir::*;
use hyperdrive_ruby::VALUE;
use std::mem::transmute;
use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_module::*;
use cranelift_simplejit::*;

use trace_compiler::TraceCompiler;
use cranelift_codegen::ir::types::I64;

use hyperdrive_ruby::rb_ary_resurrect;

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

    pub fn compile(&mut self){
        let mut codegen_context = Context::new();
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        let mut simplejit = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        simplejit.symbol("_rb_ary_resurrect", rb_ary_resurrect as *const u8);
        simplejit.symbol("_rb_str_strlen", rb_str_strlen as *const u8);
        let mut module = Module::new(simplejit);

        let sig = Signature {
            params: vec![AbiParam::new(I64)],
            returns: vec![AbiParam::new(I64)],
            call_conv: CallConv::SystemV,
        };
        module.declare_function("_rb_ary_resurrect", Linkage::Import, &sig).unwrap();
        module.declare_function("_rb_str_strlen", Linkage::Import, &sig).unwrap();

        let func_id = module
            .declare_function(&self.anchor.to_string(), Linkage::Export, &codegen_context.func.signature)
            .expect("CraneLift error declaring function");

        {
            let mut builder_context = FunctionBuilderContext::new();
            let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
            let mut compiler = TraceCompiler::new(&mut module, builder);
            compiler.compile(self.nodes.clone());
        }

        module
            .define_function(func_id, &mut codegen_context)
            .expect("CraneLift error defining function");

        module.finalize_definitions();
            let compiled_code = module.get_finalized_function(func_id);

        self.compiled_code = Some(unsafe { transmute::<_, fn(*const VALUE) -> i64>(compiled_code) });
    }

    pub fn preview(&mut self) -> String {
        let mut codegen_context = Context::new();
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        let mut module = Module::new(SimpleJITBuilder::new(cranelift_module::default_libcall_names()));
        let _func_id = module
            .declare_function("test", Linkage::Export, &codegen_context.func.signature)
            .expect("CraneLift error declaring function");

        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = TraceCompiler::new(&mut module, builder);
        compiler.compile(self.nodes.clone());
        compiler.preview().unwrap()
    }
}
