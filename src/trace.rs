use std::cell::Cell;
use cranelift::prelude::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;
use hyperdrive_ruby::VALUE;
use ir::*;
use std::fmt;
use std::mem::transmute;
use vm::*;

use compiler::Compiler;

pub type IrNodes = Vec<IrNode>;

#[derive(Clone)]
pub struct Trace {
    pub nodes: IrNodes,
    pub anchor: u64,
    pub compiled_code: Option<fn(*const VALUE, *const VALUE, VALUE) -> *const IrNode>,
}

impl Trace {
    pub fn new(nodes: IrNodes, thread: Thread) -> Self {
        Trace {
            anchor: thread.get_pc() as u64,
            nodes: nodes,
            compiled_code: None,
        }
    }

    pub fn compile(&mut self, module: &mut Module<SimpleJITBackend>) {
        let mut codegen_context = Context::new();
        let func_id = self.declare(&mut codegen_context, module);
        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = Compiler::new(module, builder);
        compiler.compile(self);

        module
            .define_function(func_id, &mut codegen_context)
            .expect("CraneLift error defining function");

        module.finalize_definitions();
        let compiled_code = module.get_finalized_function(func_id);

        self.compiled_code = Some(unsafe { transmute::<_, fn(*const VALUE, *const VALUE, VALUE) -> *const IrNode>(compiled_code) });
    }

    pub fn preview(&mut self, module: &mut Module<SimpleJITBackend>) -> String {
        let mut codegen_context = Context::new();
        self.declare(&mut codegen_context, module);
        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = Compiler::new(module, builder);
        compiler.compile(self);
        compiler.preview().unwrap()
    }

    fn declare(&mut self, codegen_context: &mut Context, module: &mut Module<SimpleJITBackend>) -> FuncId {
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        // Thread, base_bp, SP, self
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));

        // Exit Node
        codegen_context.func.signature.returns.push(AbiParam::new(types::I64));

        module
            .declare_function(
                &self.anchor.to_string(),
                Linkage::Export,
                &codegen_context.func.signature,
            )
            .expect("CraneLift error declaring function")
    }
}

impl fmt::Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.nodes.iter().enumerate()).finish()
    }
}
