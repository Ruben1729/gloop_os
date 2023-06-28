use std::vec::Vec;
use std::string::String;
use std::sync::{Arc};
use spin::Mutex as SpinMutex;

pub type INodeRef = Option<Arc<SpinMutex<INode>>>;

#[derive(PartialEq)]
pub enum INodeType {
    File,
    Directory
}

pub struct INode {
    pub name: String,
    pub size: usize,
    pub node_type: INodeType,
    pub file_ptr: usize,
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
