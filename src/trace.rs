use ir::*;

pub struct Trace {
    pub nodes: Vec<IrNode>,
    pub anchor: u64,
}

impl Trace {
    pub fn add_node(&mut self, node: IrNode){
        self.nodes.push(node);
    }
}
