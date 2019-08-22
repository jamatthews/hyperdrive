use cranelift_codegen::ir::types::I64;
use cranelift::prelude::*;
use cranelift_module::*;
use cranelift_simplejit::*;

use ir::*;
use yarv_opcode::*;

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
        let slot = self.builder.create_stack_slot(StackSlotData{kind: StackSlotKind::ExplicitSlot, size: 8, offset: None});
        let entry_block = self.builder.create_ebb();
        let loop_block = self.builder.create_ebb();
        self.builder.switch_to_block(entry_block);
        let result = self.builder.ins().iconst(I64, 0 as i64);
        self.builder.ins().stack_store(result, slot, 0);
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
                    stack.push( self.builder.ins().stack_load(I64, slot, 0) );
                },
                OpCode::Yarv(YarvOpCode::setlocal_WC_0) => {
                    self.builder.ins().stack_store(stack.pop().unwrap(), slot, 0);
                },
                OpCode::Yarv(YarvOpCode::putobject) => {
                    stack.push(self.builder.ins().iconst(I64, 1002 as i64));
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
        let val = self.builder.ins().stack_load(I64, slot, 0);
        self.builder.ins().return_(&[val]);
    }

    pub fn preview(&mut self) -> Result<String, String> {
        Ok(self.builder.display(None).to_string())
    }
}
