use crate::types::{KeyT, ValT};
use crate::indexes::treap::node::{Node, NodeRef};
use std::rc::Rc;
use std::cell::RefCell;

use super::super::abstract_index::Index;

pub struct TreapIndex {
    root: NodeRef<KeyT, ValT>,
}

impl TreapIndex {
    pub fn new() -> Self {
        TreapIndex { 
            root: None 
        }
    }

    // Helper function for right rotation
    fn rotate_right(node: Rc<RefCell<Node<KeyT, ValT>>>) -> NodeRef<KeyT, ValT> {
        let mut node_borrowed = node.borrow_mut();
        let left_child = node_borrowed.left.take();

        if let Some(left_rc) = left_child.clone() {
            let mut left_borrowed = left_rc.borrow_mut();
            node_borrowed.left = left_borrowed.right.take();
            left_borrowed.right = Some(node.clone());
            Some(left_rc.clone())
        } else {
            Some(node.clone())
        }
    }

    // Helper function for left rotation
    fn rotate_left(node: Rc<RefCell<Node<KeyT, ValT>>>) -> NodeRef<KeyT, ValT> {
        let mut node_borrowed = node.borrow_mut();
        let right_child = node_borrowed.right.take();

        if let Some(right_rc) = right_child.clone() {
            let mut right_borrowed = right_rc.borrow_mut();
            node_borrowed.right = right_borrowed.left.take();
            right_borrowed.left = Some(node.clone());
            Some(right_rc.clone())
        } else {
            Some(node.clone())
        }
    }

    // Adjusts the tree to maintain the Treap properties after insertion or updates
    fn balance_node(&mut self, node_rc: Rc<RefCell<Node<KeyT, ValT>>>) -> NodeRef<KeyT, ValT> {
        let node = node_rc.borrow();
        if let Some(left_rc) = &node.left {
            if left_rc.borrow().priority > node.priority {
                drop(node);     // Release the borrow to avoid panic during rotation
                return Self::rotate_right(node_rc);
            }
        }
        if let Some(right_rc) = &node.right {
            if right_rc.borrow().priority > node.priority {
                drop(node);     // Release the borrow to avoid panic during rotation
                return Self::rotate_left(node_rc);
            }
        }
        drop(node);
        Some(node_rc.clone())
    }

    // Insert function
    pub fn insert_node(&mut self, key: KeyT, value: ValT, priority: u64) {
        self.root = Self::insert(self.root.take(), key, value, priority);
    }

    fn insert(
        node: NodeRef<KeyT, ValT>,
        key: KeyT,
        value: ValT,
        priority: u64,
    ) -> NodeRef<KeyT, ValT> {
        if let Some(n) = node {
            let mut n_borrow = n.borrow_mut();
            if key < n_borrow.key {
                n_borrow.left = Self::insert(n_borrow.left.take(), key.clone(), value.clone(), priority);
                if let Some(left_node) = &n_borrow.left {
                    if left_node.borrow().priority > n_borrow.priority {
                        return Self::rotate_right(Rc::clone(&n));
                    }
                }
            } else if key > n_borrow.key {
                n_borrow.right = Self::insert(n_borrow.right.take(), key.clone(), value.clone(), priority);
                if let Some(right_node) = &n_borrow.right {
                    if right_node.borrow().priority > n_borrow.priority {
                        return Self::rotate_left(Rc::clone(&n));
                    }
                }
            } else {
                n_borrow.value = value;
            }
            Some(Rc::clone(&n))
        } else {
            Some(Node::new(key, value))
        }
    }

    pub fn get_node(&mut self, key: &KeyT) -> Option<&ValT> {
        let mut current = self.root;
        loop {
            match current {
                Some (node_rc) => {
                    if key < &node.key {
                        current = node.left;
                    } else if key > &node.key {
                        current = node.right;
                    } else {
                        // Found the node, increment its priority
                        node.increment_priority();
                        // Balance the node after updating its priority
                        drop(node); // Release the borrow before balancing
                        current = self.balance_node(node_rc);
                        return current.value;
                    }        
                }
                _ => break;
            }
        }
        
        None

        // while let Some(node_rc) = current {
        //     let mut node = node_rc.borrow_mut();
        //     if key < &node.key {
        //         current = node.left.clone();
        //     } else if key > &node.key {
        //         current = node.right.clone();
        //     } else {
        //         // Found the node, increment its priority
        //         node.increment_priority();
        //         // Balance the node after updating its priority
        //         drop(node); // Release the borrow before balancing
        //         current = self.balance_node(node_rc);
        //         return current.value;
        //     }
        // }
        // None
    }
}

impl Index for TreapIndex {
    fn insert(&mut self, key: KeyT, val: ValT) {
        self.insert_node(key, val, 0);
    }

    fn get(&self, key: &KeyT) -> Option<&ValT> {
        return self.get_node(key);
        // match self.get_node(key) with {
            
        // }
        // if let Some(node_rc) = self.get_node(key) {
        //     let node = node_rc.borrow();
        //     return Some(&node.value.clone());
        // }
        // None
    }
}