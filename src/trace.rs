use cranelift::prelude::*;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_module::*;
use cranelift_simplejit::*;
use hyperdrive_ruby::VALUE;
use ir;
use ir::*;
use std::collections::HashMap;
use std::fmt;
use std::mem::transmute;
use vm;
use vm::*;

use compiler::Compiler;

pub type IrNodes = Vec<IrNode>;

#[derive(Clone)]
pub struct Trace {
    pub nodes: IrNodes,
    pub anchor: u64,
    pub compiled_code: Option<fn(*const VALUE, *const VALUE, *const *mut u64) -> u64>,
    pub self_: VALUE,
    pub sp_base: u64,
}

impl Trace {
    pub fn new(nodes: IrNodes, thread: Thread) -> Self {
        Trace {
            anchor: thread.get_pc() as u64,
            nodes: nodes,
            compiled_code: None,
            self_: thread.get_self(),
            sp_base: thread.get_sp() as u64,
        }
    }

    pub fn compile(&mut self, module: &mut Module<SimpleJITBackend>) {
        let mut codegen_context = Context::new();
        let func_id = self.declare(&mut codegen_context, module);
        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = Compiler::new(module, builder);
        compiler.compile(self.clone());

        module
            .define_function(func_id, &mut codegen_context)
            .expect("CraneLift error defining function");

        module.finalize_definitions();
        let compiled_code = module.get_finalized_function(func_id);

        self.compiled_code =
            Some(unsafe { transmute::<_, fn(*const VALUE, *const VALUE, *const *mut u64) -> u64>(compiled_code) });
    }

    pub fn preview(&mut self, module: &mut Module<SimpleJITBackend>) -> String {
        let mut codegen_context = Context::new();
        self.declare(&mut codegen_context, module);
        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut codegen_context.func, &mut builder_context);
        let mut compiler = Compiler::new(module, builder);
        compiler.compile(self.clone());
        compiler.preview().unwrap()
    }

    fn declare(&mut self, codegen_context: &mut Context, module: &mut Module<SimpleJITBackend>) -> FuncId {
        codegen_context.func.signature.call_conv = CallConv::SystemV;
        // Thread, EP, SP
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        codegen_context.func.signature.params.push(AbiParam::new(types::I64));
        // PC
        codegen_context.func.signature.returns.push(AbiParam::new(types::I64));

        module
            .declare_function(
                &self.anchor.to_string(),
                Linkage::Export,
                &codegen_context.func.signature,
            )
            .expect("CraneLift error declaring function")
    }

    pub fn peel(&mut self) {
        let peeled = self.nodes.clone();
        let base_snap = match &self.nodes.last().unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing after snapshot")
        };
        self.nodes.push(IrNode {
            type_: IrType::None,
            opcode: ir::OpCode::Loop,
            operands: vec![],
            ssa_operands: vec![],
        });
        let offset = peeled.len() + 1;
        for node in &peeled {
            let opcode = match &node.opcode {
                ir::OpCode::Guard(type_, snap) => ir::OpCode::Guard(type_.clone(), self.copy_snapshot(snap, offset)),
                ir::OpCode::Snapshot(snap) => ir::OpCode::Snapshot(self.copy_snapshot(snap, offset)),
                //problem here: replacing setlocal followed by getlocal won't respect the setlocal because we don't have the stack map
                ir::OpCode::Yarv(vm::OpCode::getlocal_WC_0) => ir::OpCode::Pass(*base_snap.get(&(node.operands[0] as isize * -8)).expect("missing stack map for repeated getlocal")),
                op => op.clone(),
            };
            self.nodes.push(IrNode {
                type_: node.type_.clone(),
                opcode: opcode,
                operands: node.operands.clone(),
                ssa_operands: node.ssa_operands.iter().map(|op| *op + peeled.len() + 1).collect(),
            });
        }
        self.phi(peeled.len() - 1);
    }

    fn copy_snapshot(&self, snap: &Snapshot, bias: usize) -> Snapshot {
        let mut updated = HashMap::new();
        for (offset, ssa_ref) in snap.stack_map.iter() {
            updated.insert(offset.clone(), ssa_ref + bias);
        }
        Snapshot {
            pc: snap.pc,
            sp: snap.sp,
            self_: snap.self_.clone(),
            stack_map: updated,
        }
    }

    pub fn phi(&mut self, idx: usize) {
        let after = match &self.nodes.last().unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing after snapshot")
        };
        let before = match &self.nodes.get(idx).unwrap().opcode {
            ir::OpCode::Snapshot(s) => s.stack_map.clone(),
            _ => panic!("missing before snapshot")
        };

        for (slot, ssa_ref) in after.iter() {
            if before.get(slot) != Some(ssa_ref) {
                self.nodes.push(IrNode {
                    type_: IrType::None,
                    opcode: ir::OpCode::Phi,
                    operands: vec![],
                    ssa_operands: vec![*before.get(slot).unwrap(), *ssa_ref],
                });
            }
        }
    }
}

impl fmt::Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.nodes.iter().enumerate()).finish()
    }
}
