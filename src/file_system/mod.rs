use std::string::String;
use std::vec::Vec;
use lazy_static::lazy_static;
use crate::file_system::inode::{INodeType, INode, INodeRef};

use spin::Mutex as SpinMutex;
use crate::println;

pub mod inode;
pub mod file_descriptor;

pub struct FileSystem {
    root: INodeRef,
    current_node: INodeRef,
    cap: usize
}

impl FileSystem {
    pub fn new(capacity: usize) -> Self {
        let root_node = INode::new(String::from("root"), INodeType::Directory);

        FileSystem {
            root: root_node.clone(),
            current_node: root_node,
            cap: capacity,
        }
    }

    pub fn create_node(&mut self, name: String, node_type: INodeType) {
        if let Some(parent) = &self.current_node {
            let new_node = INode::new(name, node_type);
            let mut locked_parent = parent.lock();
            locked_parent.add_child(new_node);
        }
    }
}

lazy_static! {
    pub static ref FILE_SYSTEM: SpinMutex<FileSystem> = SpinMutex::new(FileSystem::new(100));
}

pub fn create_file(arguments: Vec<&str>) -> Result<(), &'static str> {
    if let Some(node_name) = arguments.get(0) {
        FILE_SYSTEM.lock().create_node(String::from(*node_name), INodeType::File);
    }

    Ok(())
}

pub fn create_folder(arguments: Vec<&str>) -> Result<(), &'static str> {
    if let Some(node_name) = arguments.get(0) {
        FILE_SYSTEM.lock().create_node(String::from(*node_name), INodeType::Directory);
    }

    Ok(())
}

pub fn change_directory(arguments: Vec<&str>) -> Result<(), &'static str> {
    let mut new_node: Option<INodeRef> = None;

    if let Some(directory_name) = arguments.get(0) {
        if directory_name.eq(&"..") {
            todo!()
        }

        for child in FILE_SYSTEM.lock().current_node.clone().unwrap().lock().children.iter() {
            if child.clone().unwrap().lock().node_type == INodeType::File {
                return Err("Can't navigate into files.");
            }

            if child.clone().unwrap().lock().name.eq(directory_name) {
                new_node = Some(child.clone());
            }
        }
    }

    if let Some(node) = new_node {
        FILE_SYSTEM.lock().current_node = node;
        Ok(())
    } else {
        Err("Unable to find directory with that name.")
    }
}

pub fn list_inodes(arguments: Vec<&str>) -> Result<(), &'static str> {
    for child in FILE_SYSTEM.lock().current_node.clone().unwrap().lock().children.iter() {
        println!("- {}", child.clone().unwrap().lock().name);
    }

    Ok(())
}
