use hyperdrive_ruby::ruby_special_consts_RUBY_Qnil;
use ir::OpCode::Snapshot;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::ir::types::I64;
use cranelift::prelude::*;
use cranelift_module::*;
use cranelift_simplejit::*;
use cranelift_module::FuncOrDataId::Func;
use ir::*;
use yarv_opcode::*;
use yarv_types::*;
use vm_call_cache::*;

macro_rules! b1_2_value {
    ($x:ident, $builder:ident) => {{
        let fifth_bit = $builder.ins().bint(I64, $x);
        let fifth_bit = $builder.ins().ishl_imm(fifth_bit, 4);
        let third_bit = $builder.ins().bint(I64, $x);
        let third_bit = $builder.ins().ishl_imm(third_bit, 2);
        $builder.ins().iadd(fifth_bit, third_bit)
    }}
}

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
    builder: FunctionBuilder<'a>
}

impl <'a> TraceCompiler<'a> {

    pub fn new(module: &'a mut Module<SimpleJITBackend>, builder: FunctionBuilder<'a>) -> Self {
        Self {
            module: module,
            builder: builder,
        }
    }

    pub fn compile(&mut self, trace: Vec<IrNode>){
        //println!("{:#?}", trace);
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
                OpCode::Yarv(YarvOpCode::putnil) => {
                    stack.push(self.builder.ins().iconst(I64, ruby_special_consts_RUBY_Qnil as i64));
                },
                OpCode::Yarv(YarvOpCode::putobject_INT2FIX_1_) => {
                    stack.push(self.builder.ins().iconst(I64, 1 as i64));
                },
                OpCode::Yarv(YarvOpCode::putobject_INT2FIX_0_) => {
                    stack.push(self.builder.ins().iconst(I64, 0 as i64));
                },
                OpCode::Yarv(YarvOpCode::opt_plus) => {
                    let b = stack.pop().expect("stack underflow in opt_plus");
                    let a = stack.pop().expect("stack underflow in opt_plus");
                    stack.push(self.builder.ins().iadd(a, b));
                },
                OpCode::Yarv(YarvOpCode::getlocal_WC_0) => {
                    let offset = -8 * node.operands[0] as i32;
                    match node.type_ {
                        IrType::Yarv(ValueType::Fixnum) => {
                            let boxed = self.builder.ins().load(I64, MemFlags::new(), ep, offset);
                            let builder = &mut self.builder;
                            let unboxed = value_2_i64!(boxed, builder);
                            stack.push(unboxed);
                        },
                        IrType::Yarv(ValueType::Array) => {
                            let boxed = self.builder.ins().load(I64, MemFlags::new(), ep, offset);
                            stack.push(boxed);
                        },
                        _ => panic!("unexpected: type {:?} in getlocal at offset: {} \n {:#?}", node.type_, i, trace),
                    }
                },
                OpCode::Yarv(YarvOpCode::setlocal_WC_0) => {
                    let offset = -8 * node.operands[0] as i32;
                    match node.type_ {
                        IrType::Internal(InternalType::I64) => {
                            let unboxed = stack.pop().expect("stack underflow in setlocal");
                            let builder = &mut self.builder;
                            let rvalue = i64_2_value!(unboxed, builder);
                            self.builder.ins().store(MemFlags::new(), rvalue, ep,  offset);
                        },
                        IrType::Yarv(ValueType::Array) => {
                            let rvalue = stack.pop().expect("stack underflow in setlocal");
                            self.builder.ins().store(MemFlags::new(), rvalue, ep,  offset);
                        },
                        IrType::Internal(InternalType::Bool) => {
                            let unboxed = stack.pop().expect("stack underflow in setlocal");
                            let builder = &mut self.builder;
                            let rvalue = b1_2_value!(unboxed, builder);
                            self.builder.ins().store(MemFlags::new(), rvalue, ep,  offset);
                        }
                        _ => panic!("unexpect type {:?} in setlocal", node.type_),
                    };
                },
                OpCode::Yarv(YarvOpCode::putstring) => {
                    let unboxed = node.operands[0];
                    stack.push(self.builder.ins().iconst(I64, unboxed as i64));
                },
                OpCode::Yarv(YarvOpCode::putobject) => {
                    let unboxed = node.operands[0] >> 1;
                    stack.push(self.builder.ins().iconst(I64, unboxed as i64));
                },
                OpCode::Yarv(YarvOpCode::opt_lt) => {
                    let b = stack.pop().expect("stack underflow in opt_lt");
                    let a = stack.pop().expect("stack underflow in opt_lt");
                    let result = self.builder.ins().icmp(IntCC::SignedLessThan, a, b);
                    stack.push(result);
                },
                OpCode::Yarv(YarvOpCode::opt_eq) => {
                    let b = stack.pop().expect("stack underflow in opt_eq");
                    let a = stack.pop().expect("stack underflow in opt_eq");
                    let result = self.builder.ins().icmp(IntCC::Equal, a, b);
                    stack.push(result);
                },
                OpCode::Yarv(YarvOpCode::branchif) => {
                    let a = stack.pop().expect("stack underflow in branchif");

                    if i == trace.len() - 2 {
                        self.builder.ins().brnz(a, loop_block, &[]);
                    } else {
                        let loop_block = self.builder.create_ebb();
                        let side_exit_block = self.builder.create_ebb();
                        self.builder.ins().brnz(a, side_exit_block, &[]);
                        self.builder.ins().jump(loop_block, &[]);
                        self.builder.switch_to_block(side_exit_block);
                        self.builder.ins().return_(&[]);
                        self.builder.switch_to_block(loop_block);
                    }

                },
                OpCode::Yarv(YarvOpCode::branchunless) => {
                    let a = stack.pop().expect("stack underflow in branchif");
                    if i == trace.len() - 2 {
                        self.builder.ins().brnz(a, loop_block, &[]);
                    } else {
                        let loop_block = self.builder.create_ebb();
                        let side_exit_block = self.builder.create_ebb();
                        self.builder.ins().brnz(a, side_exit_block, &[]);
                        self.builder.ins().jump(loop_block, &[]);
                        self.builder.switch_to_block(side_exit_block);
                        self.builder.ins().return_(&[]);
                        self.builder.switch_to_block(loop_block);
                    }

                },
                OpCode::Yarv(YarvOpCode::duparray) => {
                    let array = node.operands[0];
                    let array = self.builder.ins().iconst(I64, array as i64);
                    if let Some(Func(id)) = self.module.get_name("_rb_ary_resurrect") {
                        let func_ref = self.module.declare_func_in_func(id, self.builder.func);
                        let call = self.builder.ins().call(func_ref, &[array]);
                        let result = self.builder.inst_results(call)[0];
                        stack.push(result);
                    } else {
                        panic!("function not found!");
                    }

                },
                OpCode::Yarv(YarvOpCode::opt_send_without_block) => {
                    let call_cache = VmCallCache::new(node.operands[1] as *const _);
                    let func = call_cache.get_func() as *const u64;
                    let func = self.builder.ins().iconst(I64, func as i64);
                    let receiver = stack.pop().expect("stack underflow in send");

                    let sig = Signature {
                        params: vec![AbiParam::new(I64)],
                        returns: vec![AbiParam::new(I64)],
                        call_conv: CallConv::SystemV,
                    };
                    let sig = self.builder.import_signature(sig);

                    let call = self.builder.ins().call_indirect(sig, func, &[receiver]);
                    let result = self.builder.inst_results(call)[0];
                    stack.push(result);
                },
                OpCode::Yarv(YarvOpCode::opt_empty_p) => {
                    let receiver = stack.pop().expect("stack underflow in send");

                    if let Some(Func(id)) = self.module.get_name("_rb_str_strlen") {
                        let func_ref = self.module.declare_func_in_func(id, self.builder.func);
                        let call = self.builder.ins().call(func_ref, &[receiver]);
                        let count = self.builder.inst_results(call)[0];
                        let result = self.builder.ins().icmp_imm(IntCC::Equal, count, 0);
                        stack.push(result);
                    } else {
                        panic!("function not found!");
                    }

                },
                OpCode::Yarv(YarvOpCode::pop) => {
                    stack.pop().expect("stack underflow in pop");
                },
                OpCode::Yarv(YarvOpCode::dup) => {
                    let unboxed = node.operands[0];
                    stack.push(self.builder.ins().iconst(I64, unboxed as i64));
                },
                OpCode::Yarv(YarvOpCode::opt_not) => {
                    let val = stack.pop().expect("stack underflow in branchif");
                    let result = self.builder.ins().bnot(val);
                    stack.push(result);
                },
                Snapshot(_) => {},
                _ => panic!("NYI: {:?}", node.opcode),
            };
        };
        self.builder.ins().return_(&[]);
    }

    pub fn preview(&mut self) -> Result<String, String> {
        Ok(self.builder.display(None).to_string())
    }
}
