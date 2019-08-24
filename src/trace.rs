use ir::*;
use vm_thread::VmThread;
use hyperdrive_ruby::VALUE;
use std::mem::transmute;
use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_module::*;
use cranelift_simplejit::*;

use trace_compiler::TraceCompiler;

use instruction_recorder::record_instruction;

pub type IrNodes = Vec<IrNode>;

#[derive(Clone, Debug)]
pub struct Trace {
    pub nodes: IrNodes,
    pub start: u64,
    pub exit: u64,
    pub compiled_code: Option<fn(*const VALUE) -> i64>,
}

impl Trace {
    pub fn new(pc: u64) -> Self {
        Trace {
            start: pc,
            exit: pc,
            nodes: vec![],
            compiled_code: None,
        }
    }

    pub fn complete(&mut self) {
        self.nodes.push(
            IrNode {
                type_: IrType::Snapshot,
                opcode: OpCode::Snapshot(self.exit + 8),
                operands: vec![],
            }
        );
    }

    pub fn record_instruction(&mut self, thread: VmThread) {
        self.exit = thread.get_pc() as u64;
        record_instruction(&mut self.nodes, thread);
    }

    pub fn compile(&mut self){
        let mut codegen_context = Context::new();
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        let mut module = Module::new(SimpleJITBuilder::new(cranelift_module::default_libcall_names()));
        let func_id = module
            .declare_function("test", Linkage::Export, &codegen_context.func.signature)
            .expect("CraneLift error declaring function");

        {
            let mut builder_context = FunctionBuilderContext::new();
            let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
            TraceCompiler::new(&mut module, builder).compile(self.nodes.clone());
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
