use std::vec::Vec;
use std::string::String;
use std::sync::{Arc};
use spin::Mutex as SpinMutex;

pub type INodeRef = Option<Arc<SpinMutex<INode>>>;

pub enum INodeType {
    File,
    Directory
}

pub struct INode {
    pub name: String,
    size: usize,
    node_type: INodeType,
    file_ptr: usize,
    pub children: Vec<INodeRef>
}

impl INode {
    pub fn new(name: String, node_type: INodeType) -> INodeRef {
        Some(Arc::new(SpinMutex::new(INode {
            name,
            size: 0,
            node_type,
            file_ptr: 0,
            children: Vec::new()
        })))
    }

    pub fn add_child(&mut self, new_node: INodeRef) {
        self.children.push(new_node);
    }
}
