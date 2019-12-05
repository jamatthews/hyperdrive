use cranelift::prelude::*;
use cranelift_codegen::ir::types::I64;
use cranelift_codegen::isa::CallConv;
use cranelift_module::FuncOrDataId::Func;
use cranelift_module::*;
use cranelift_simplejit::*;
use hyperdrive_ruby::ruby_special_consts_RUBY_Qnil;
use ir::OpCode;
use ir::OpCode::Snapshot;
use ir::*;
use trace::Trace;
use vm;
use vm::*;

macro_rules! b1_2_value {
    ($x:ident, $builder:ident) => {{
        let fifth_bit = $builder.ins().bint(I64, $x);
        let fifth_bit = $builder.ins().ishl_imm(fifth_bit, 4);
        let third_bit = $builder.ins().bint(I64, $x);
        let third_bit = $builder.ins().ishl_imm(third_bit, 2);
        $builder.ins().iadd(fifth_bit, third_bit)
    }};
}

macro_rules! value_2_i64 {
    ($x:ident, $builder:ident) => {{
        $builder.ins().ushr_imm($x, 1)
    }};
}

macro_rules! i64_2_value {
    ($x:ident, $builder:ident) => {{
        let value = $builder.ins().ishl_imm($x, 1);
        $builder.ins().iadd_imm(value, 1)
    }};
}

pub struct Compiler<'a> {
    module: &'a mut Module<SimpleJITBackend>,
    builder: FunctionBuilder<'a>,
    ssa_values: Vec<cranelift::prelude::Value>,
}

impl<'a> Compiler<'a> {
    pub fn new(module: &'a mut Module<SimpleJITBackend>, builder: FunctionBuilder<'a>) -> Self {
        Self {
            module: module,
            builder: builder,
            ssa_values: vec![],
        }
    }

    pub fn compile(&mut self, trace: Trace) {
        let entry_block = self.builder.create_ebb();
        self.builder.switch_to_block(entry_block);
        self.builder.append_ebb_params_for_function_params(entry_block);
        let _thread = self.builder.ebb_params(entry_block)[0];
        let ep = self.builder.ebb_params(entry_block)[1];
        let sp_ptr = self.builder.ebb_params(entry_block)[2];
        let self_ = self.builder.ebb_params(entry_block)[3];

        let partition = trace
            .nodes
            .iter()
            .position(|node| match node.opcode {
                OpCode::Loop => true,
                _ => false,
            })
            .expect("no LOOP opcode");

        self.translate_nodes(trace.nodes[..partition].to_vec(), trace.clone(), ep, sp_ptr, self_);

        let loop_start = self.builder.create_ebb();
        let phis: Vec<&IrNode> = trace.nodes.iter().filter(|n| n.opcode == OpCode::Phi).collect();

        // take all the SSA Values from the prelude to put into the first call of the repeating block
        let phi_params: Vec<_> = phis
            .iter()
            .map(|n| n.ssa_operands[0])
            .map(|r| self.ssa_values[r])
            .collect();
        self.builder.ins().jump(loop_start, &phi_params);

        //put the EBB params into ssa_values[i] so we actually use the param (which is initially the value anyway)
        for (i, node) in phis.iter().enumerate() {
            self.builder.append_ebb_param(loop_start, I64);
            let replacing = node.ssa_operands[0];
            self.ssa_values[replacing] = self.builder.ebb_params(loop_start)[i];
        }
        self.builder.switch_to_block(loop_start);

        self.translate_nodes(trace.nodes[partition..].to_vec(), trace.clone(), ep, sp_ptr, self_);

        //jumping back to the loop we use the dominating values from the right hand side of the PHI node
        let phi_params: Vec<_> = phis
            .iter()
            .map(|n| n.ssa_operands[1])
            .map(|r| self.ssa_values[r])
            .collect();
        self.builder.ins().jump(loop_start, &phi_params);
    }

    pub fn preview(&mut self) -> Result<String, String> {
        Ok(self.builder.display(None).to_string())
    }

    fn translate_nodes(
        &mut self,
        nodes: Vec<IrNode>,
        trace: Trace,
        ep: cranelift::prelude::Value,
        sp_ptr: cranelift::prelude::Value,
        self_: cranelift::prelude::Value
    ) {
        for node in nodes.iter() {
            match &node.opcode {
                OpCode::LoadSelf => {
                    self.ssa_values.push(self_);
                }
                OpCode::Pass(ssa_ref) => {
                    let passthrough = self.ssa_values[*ssa_ref];
                    self.ssa_values.push(passthrough);
                }
                OpCode::Phi
                | OpCode::Loop
                | Snapshot(_)
                | OpCode::Yarv(vm::OpCode::putnil) => {
                    self.putconstant(ruby_special_consts_RUBY_Qnil as i64);
                }
                OpCode::Yarv(vm::OpCode::putobject_INT2FIX_1_) => self.putconstant(1),
                OpCode::Yarv(vm::OpCode::putobject_INT2FIX_0_) => self.putconstant(0),
                OpCode::Yarv(vm::OpCode::opt_plus) => self.binary_op(node),
                OpCode::StackLoad(offset) => {
                    let boxed = self.builder.ins().load(I64, MemFlags::new(), ep, *offset as i32);
                    let unboxed = self.unbox(boxed, &node);
                    self.ssa_values.push(unboxed);
                }
                OpCode::Yarv(vm::OpCode::setlocal_WC_0) => {
                    let offset = -8 * node.operands[0] as i32;
                    let ssa_ref = node.ssa_operands[0];
                    let unboxed = self.ssa_values[ssa_ref];
                    let boxed = self.box_(unboxed, &trace.nodes[ssa_ref]);

                    self.builder.ins().store(MemFlags::new(), boxed, ep, offset);

                    //push to keep alignment
                    self.ssa_values.push(self.ssa_values[ssa_ref]);
                }
                OpCode::Yarv(vm::OpCode::putstring) => {
                    self.putconstant(node.operands[0] as i64);
                }
                OpCode::Yarv(vm::OpCode::putobject) => {
                    let boxed = node.operands[0];
                    let boxed = self.builder.ins().iconst(I64, boxed as i64);
                    let unboxed = self.unbox(boxed, &node);
                    self.ssa_values.push(unboxed);
                }
                OpCode::Yarv(vm::OpCode::opt_lt) => self.binary_op(node),
                OpCode::Yarv(vm::OpCode::opt_eq) => self.binary_op(node),
                OpCode::Guard(IrType::Yarv(type_), snapshot) => {
                    let value = self.ssa_values[node.ssa_operands[0]];
                    let ssa_ref = node.ssa_operands[0];
                    let side_exit_block = self.builder.create_ebb();

                    match type_ {
                        ValueType::True => self.builder.ins().brz(value, side_exit_block, &[]),
                        ValueType::False => self.builder.ins().brnz(value, side_exit_block, &[]),
                        _ => panic!(
                            "unexpect type {:?} in guard\n {:#?} ",
                            trace.nodes[ssa_ref].type_, trace.nodes
                        ),
                    };

                    let continue_block = self.builder.create_ebb();
                    self.builder.ins().jump(continue_block, &[]);
                    self.builder.switch_to_block(side_exit_block);

                    for (offset, ssa_ref) in snapshot.stack_map.iter() {
                        let boxed = self.box_(self.ssa_values[*ssa_ref], &trace.nodes[*ssa_ref]);
                        self.builder.ins().store(MemFlags::new(), boxed, ep, *offset as i32);
                    }

                    let sp = self.builder.ins().iconst(I64, snapshot.sp as i64);
                    self.builder.ins().store(MemFlags::new(), sp, sp_ptr, 0);
                    let pc = self.builder.ins().iconst(I64, snapshot.pc as i64);
                    self.builder.ins().return_(&[pc]);

                    self.builder.switch_to_block(continue_block);
                    self.ssa_values.push(self.ssa_values[ssa_ref]);
                }
                OpCode::Yarv(vm::OpCode::duparray) => {
                    let array = node.operands[0];
                    let array = self.builder.ins().iconst(I64, array as i64);
                    self.internal_call_push("_rb_ary_resurrect", &[array]);
                }
                OpCode::Yarv(vm::OpCode::opt_send_without_block) => {
                    let call_cache = CallCache::new(node.operands[1] as *const _);
                    let func = call_cache.get_func() as *const u64;
                    let func = self.builder.ins().iconst(I64, func as i64);
                    let receiver = self.ssa_values[node.ssa_operands[0]];

                    let sig = Signature {
                        params: vec![AbiParam::new(I64)],
                        returns: vec![AbiParam::new(I64)],
                        call_conv: CallConv::SystemV,
                    };
                    let sig = self.builder.import_signature(sig);

                    let call = self.builder.ins().call_indirect(sig, func, &[receiver]);
                    let result = self.builder.inst_results(call)[0];
                    self.ssa_values.push(result);
                }
                OpCode::Yarv(vm::OpCode::opt_empty_p) => {
                    let receiver = self.ssa_values[node.ssa_operands[0]];
                    let count = self.internal_call("_rb_str_strlen", &[receiver]);
                    let result = self.builder.ins().icmp_imm(IntCC::Equal, count, 0);
                    self.ssa_values.push(result);
                }
                OpCode::Yarv(vm::OpCode::dup) | OpCode::Yarv(vm::OpCode::pop) => {
                    //pop actually pushes an SSA value to keep alignment
                    self.ssa_values.push(self.ssa_values[node.ssa_operands[0]])
                }
                OpCode::Yarv(vm::OpCode::opt_not) => {
                    let ssa_ref = node.ssa_operands[0];
                    let val = self.ssa_values[ssa_ref];
                    let result = match trace.nodes[ssa_ref].type_ {
                        IrType::Internal(InternalType::Bool) => self.builder.ins().bnot(val),
                        IrType::Internal(InternalType::Value) => self.builder.ins().icmp_imm(IntCC::Equal, val, 0),
                        _ => panic!(),
                    };
                    self.ssa_values.push(result);
                }
                OpCode::ArrayAppend => {
                    let array = self.ssa_values[node.ssa_operands[0]];
                    let unboxed = self.ssa_values[node.ssa_operands[1]];
                    let boxed = self.box_(unboxed, &trace.nodes[node.ssa_operands[1]]);
                    self.internal_call_push("_rb_ary_push", &[array, boxed]);
                }
                OpCode::NewArray => self.internal_call_push("_rb_ary_new", &[]),
                OpCode::ArrayGet => {
                    let array = self.ssa_values[node.ssa_operands[0]];
                    let unboxed = self.ssa_values[node.ssa_operands[1]];
                    let boxed = self.box_(unboxed, &trace.nodes[node.ssa_operands[1]]);
                    self.internal_call_push("_rb_ary_aref1", &[array, boxed]);
                }
                OpCode::ArraySet => {
                    let array = self.ssa_values[node.ssa_operands[0]];
                    let key = self.ssa_values[node.ssa_operands[1]];
                    let value = self.ssa_values[node.ssa_operands[2]];
                    let key = self.box_(key, &trace.nodes[node.ssa_operands[1]]);
                    let value = self.box_(value, &trace.nodes[node.ssa_operands[2]]);
                    self.internal_call_push("_rb_ary_store", &[array, key, value]);
                }
                OpCode::NewHash => self.internal_call_push("_rb_hash_new", &[]),
                OpCode::HashSet => {
                    let hash = self.ssa_values[node.ssa_operands[0]];
                    let key = self.ssa_values[node.ssa_operands[1]];
                    let value = self.ssa_values[node.ssa_operands[2]];
                    let key = self.box_(key, &trace.nodes[node.ssa_operands[1]]);
                    let value = self.box_(value, &trace.nodes[node.ssa_operands[2]]);
                    self.internal_call("_rb_hash_aset", &[hash, key, value]);
                    self.ssa_values.push(hash);
                }
                OpCode::HashGet => {
                    let hash = self.ssa_values[node.ssa_operands[0]];
                    let key = self.ssa_values[node.ssa_operands[1]];
                    let key = self.box_(key, &trace.nodes[node.ssa_operands[1]]);
                    self.internal_call_push("_rb_hash_aref", &[hash, key]);
                }
                _ => panic!("NYI: {:?}", node.opcode),
            };
        }
    }

    fn putconstant(&mut self, n: i64) {
        self.ssa_values.push(self.builder.ins().iconst(I64, n));
    }

    fn binary_op(&mut self, node: &IrNode) {
        let a = self.ssa_values[node.ssa_operands[0]];
        let b = self.ssa_values[node.ssa_operands[1]];
        let result = match node.opcode {
            OpCode::Yarv(vm::OpCode::opt_plus) => self.builder.ins().iadd(a, b),
            OpCode::Yarv(vm::OpCode::opt_lt) => self.builder.ins().icmp(IntCC::SignedLessThan, a, b),
            OpCode::Yarv(vm::OpCode::opt_eq) => self.builder.ins().icmp(IntCC::Equal, a, b),
            _ => panic!(),
        };
        self.ssa_values.push(result);
    }

    fn internal_call_push(&mut self, symbol: &str, args: &[cranelift::prelude::Value]) {
        let result = self.internal_call(symbol, args);
        self.ssa_values.push(result);
    }

    fn internal_call(&mut self, symbol: &str, args: &[cranelift::prelude::Value]) -> cranelift::prelude::Value {
        if let Some(Func(id)) = self.module.get_name(symbol) {
            let func_ref = self.module.declare_func_in_func(id, self.builder.func);
            let call = self.builder.ins().call(func_ref, args);
            self.builder.inst_results(call)[0]
        } else {
            panic!("internal call for {} failed: x  function not found!");
        }
    }

    fn unbox(&mut self, boxed: cranelift::prelude::Value, node: &IrNode) -> cranelift::prelude::Value {
        match node.type_ {
            //putobject might be a Fixnum but the node type is I64
            IrType::Yarv(ValueType::Fixnum) | IrType::Internal(InternalType::I64) => {
                let builder = &mut self.builder;
                value_2_i64!(boxed, builder)
            }
            IrType::Yarv(_) => boxed,
            _ => panic!(),
        }
    }

    fn box_(&mut self, unboxed: cranelift::prelude::Value, node: &IrNode) -> cranelift::prelude::Value {
        let builder = &mut self.builder;
        match node.type_ {
            IrType::Internal(InternalType::I64) => i64_2_value!(unboxed, builder),
            IrType::Internal(InternalType::Bool) => b1_2_value!(unboxed, builder),
            IrType::Yarv(_) | IrType::Internal(InternalType::Value) => unboxed,
            _ => panic!(),
        }
    }
}
