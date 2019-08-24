use hyperdrive_ruby::VALUE;
use std::mem::transmute;
use cranelift::prelude::*;
use cranelift_codegen::Context;
use cranelift_codegen::isa::CallConv;
use cranelift_module::*;
use cranelift_simplejit::*;

use trace_compiler::TraceCompiler;
use ir::IrType::Integer;
use yarv_opcode::YarvOpCode;
use ir::*;

#[derive(Clone, Debug)]
pub struct Trace {
    pub nodes: Vec<IrNode>,
    pub anchor: u64,
    pub compiled_code: Option<fn(*const VALUE) -> i64>,
}

impl Trace {
    pub fn add_node(&mut self, pc: u64, opcode: YarvOpCode){
        let node = IrNode {
            type_: Integer,
            pc: pc,
            opcode: OpCode::Yarv(opcode),
            operand_1: None,
            operand_2: None,
        };
        self.nodes.push(node);
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

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrType::*;
    use ir::OpCode::*;

    #[test]
    fn it_compiles_simple() {
        let nodes = vec![
            IrNode {
                pc: 0,
                type_: Integer,
                opcode: Yarv(YarvOpCode::getlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 1,
                type_: Integer,
                opcode: Yarv(YarvOpCode::putobject_INT2FIX_1_),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 2,
                type_: Integer,
                opcode: Yarv(YarvOpCode::opt_plus),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 3,
                type_: Integer,
                opcode: Yarv(YarvOpCode::setlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 4,
                type_: Integer,
                opcode: Yarv(YarvOpCode::getlocal_WC_0),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 5,
                type_: Integer,
                opcode: Yarv(YarvOpCode::putobject),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 6,
                type_: Integer,
                opcode: Yarv(YarvOpCode::opt_lt),
                operand_1: None,
                operand_2: None,
            },
            IrNode {
                pc: 7,
                type_: Integer,
                opcode: Yarv(YarvOpCode::branchif),
                operand_1: None,
                operand_2: None,
            },
        ];
        let mut trace = Trace {
            nodes: nodes,
            anchor: 0,
            compiled_code: None,
        };
        trace.compile();
    }
}
