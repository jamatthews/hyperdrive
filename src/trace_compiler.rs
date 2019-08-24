use cranelift_codegen::ir::types::I64;
use cranelift::prelude::*;
use cranelift_module::*;
use cranelift_simplejit::*;

use ir::*;
use yarv_opcode::*;

macro_rules! value_2_i64 {
    ($x:ident, $builder:ident) => {{
        $builder.ins().ushr_imm($x, 1)
    }}
}

macro_rules! i64_2_value {
    ($x:ident, $builder:ident) => {{
        let value = $builder.ins().ishl_imm($x, 1);
        $builder.ins().iadd_imm(value, 1)
    }}
}

pub struct TraceCompiler<'a> {
    module: &'a mut Module<SimpleJITBackend>,
    builder: FunctionBuilder<'a>,
}

impl <'a> TraceCompiler<'a> {

    pub fn new(module: &'a mut Module<SimpleJITBackend>, builder: FunctionBuilder<'a>) -> Self {
        Self {
            module: module,
            builder: builder,
        }
    }

    pub fn compile(&mut self, trace: Vec<IrNode>){
        let entry_block = self.builder.create_ebb();
        let loop_block = self.builder.create_ebb();
        self.builder.switch_to_block(entry_block);
        self.builder.append_ebb_params_for_function_params(entry_block);
        let ep = self.builder.ebb_params(entry_block)[0];
        self.builder.ins().jump(loop_block, &[]);
        self.builder.switch_to_block(loop_block);

        let mut stack = vec![];
        for (i, node) in trace.iter().enumerate() {
            match node.opcode {
                OpCode::Yarv(YarvOpCode::putobject_INT2FIX_1_) => {
                    stack.push(self.builder.ins().iconst(I64, 1 as i64));
                },
                OpCode::Yarv(YarvOpCode::opt_plus) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(self.builder.ins().iadd(a, b));
                },
                OpCode::Yarv(YarvOpCode::getlocal_WC_0) => {
                    let offset = -8 * node.operands[0] as i32;
                    let boxed = self.builder.ins().load(I64, MemFlags::new(), ep, offset);
                    let mut builder = &mut self.builder;
                    let unboxed = value_2_i64!(boxed, builder);
                    stack.push(unboxed);
                },
                OpCode::Yarv(YarvOpCode::setlocal_WC_0) => {
                    let offset = -8 * node.operands[0] as i32;
                    let unboxed = stack.pop().unwrap();
                    let builder = &mut self.builder;
                    let boxed = i64_2_value!(unboxed, builder);
                    self.builder.ins().store(MemFlags::new(), boxed, ep,  offset);
                },
                OpCode::Yarv(YarvOpCode::putobject) => {
                    let unboxed = node.operands[0] >> 1;
                    stack.push(self.builder.ins().iconst(I64, unboxed as i64));
                },
                OpCode::Yarv(YarvOpCode::opt_lt) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = self.builder.ins().icmp(IntCC::SignedLessThan, a, b);
                    stack.push(result);
                },
                OpCode::Yarv(YarvOpCode::branchif) => {
                    let a = stack.pop().unwrap();
                    self.builder.ins().brnz(a, loop_block, &[]);
                },
                _=> { }
            };
        };
        self.builder.ins().return_(&[]);
    }

    pub fn preview(&mut self) -> Result<String, String> {
        Ok(self.builder.display(None).to_string())
    }
}
