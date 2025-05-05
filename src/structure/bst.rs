use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;

#[derive(Debug)]
pub struct BstNode {
    pub key: i32,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
    pub parent: Option<Weak<RefCell<BstNode>>>,
}

impl BstNode {
    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        Rc::new(RefCell::new(Self {
            key: value,
            left: None,
            right: None,
            parent: None,
        }))
    }

    pub fn tree_insert(root: &mut Option<BstNodeLink>, value: i32) {
        let mut y: Option<BstNodeLink> = None;
        let mut x = root.clone();

        while let Some(ref current_node) = x {
            y = x.clone();
            let next = if value < current_node.borrow().key {
                current_node.borrow().left.clone()
            } else {
                current_node.borrow().right.clone()
            };
            x = next;
        }

        let z = Self::new_bst_nodelink(value);
        if let Some(ref parent_node) = y {
            z.borrow_mut().parent = Some(Rc::downgrade(parent_node));
            if value < parent_node.borrow().key {
                parent_node.borrow_mut().left = Some(z);
            } else {
                parent_node.borrow_mut().right = Some(z);
            }
        } else {
            *root = Some(z);
        }
    }

    pub fn minimum(node: &BstNodeLink) -> BstNodeLink {
        let mut current = node.clone();
        while let Some(left) = {
            let borrow = current.borrow();
            borrow.left.clone()
        } {
            current = left;
        }
        current
    }

    pub fn maximum(node: &BstNodeLink) -> BstNodeLink {
        let mut current = node.clone();
        while let Some(right) = {
            let borrow = current.borrow();
            borrow.right.clone()
        } {
            current = right;
        }
        current
    }

    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let mut current = node.clone();
        while let Some(parent_weak) = {
            let borrow = current.borrow();
            borrow.parent.clone()
        } {
            if let Some(parent) = parent_weak.upgrade() {
                current = parent;
            } else {
                break;
            }
        }
        current
    }

    pub fn transplant(u: &BstNodeLink, v: Option<BstNodeLink>) {
        let parent_weak = u.borrow().parent.clone();
        if let Some(parent_weak) = parent_weak {
            if let Some(parent) = parent_weak.upgrade() {
                let is_left = match parent.borrow().left.as_ref() {
                    Some(left) => Rc::ptr_eq(left, u),
                    None => false,
                };

                if is_left {
                    parent.borrow_mut().left = v.clone();
                } else {
                    parent.borrow_mut().right = v.clone();
                }

                if let Some(v_node) = v {
                    v_node.borrow_mut().parent = Some(Rc::downgrade(&parent));
                }
            }
        }
    }

    pub fn tree_delete(root: &mut Option<BstNodeLink>, z: &BstNodeLink) {
        if z.borrow().left.is_none() {
            Self::transplant(z, z.borrow().right.clone());
        } else if z.borrow().right.is_none() {
            Self::transplant(z, z.borrow().left.clone());
        } else {
            let right = z.borrow().right.clone().unwrap();
            let y = Self::minimum(&right);

            if !Rc::ptr_eq(&y, &right) {
                let y_right = y.borrow().right.clone();
                Self::transplant(&y, y_right);
                y.borrow_mut().right = z.borrow().right.clone();
                if let Some(ref right_child) = {
                    let y_borrow = y.borrow();
                    y_borrow.right.clone()
                } {
                    right_child.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            }

            Self::transplant(z, Some(y.clone()));
            y.borrow_mut().left = z.borrow().left.clone();
            if let Some(ref left_child) = {
                let y_borrow = y.borrow();
                y_borrow.left.clone()
            } {
                left_child.borrow_mut().parent = Some(Rc::downgrade(&y));
            }
        }

        if let Some(current_root) = root.clone() {
            *root = Some(Self::get_root(&current_root));
        }
    }

    pub fn tree_successor(node: &BstNodeLink) -> Option<BstNodeLink> {
        if let Some(right) = node.borrow().right.clone() {
            return Some(Self::minimum(&right));
        }

        let mut current = node.clone();
        while let Some(parent_weak) = {
            let borrow = current.borrow();
            borrow.parent.clone()
        } {
            if let Some(parent) = parent_weak.upgrade() {
                if parent.borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(left, &current)) {
                    return Some(parent);
                }
                current = parent;
            } else {
                break;
            }
        }
        None
    }

    pub fn depth(node: &BstNodeLink) -> usize {
        let mut depth = 0;
        let mut current = node.clone();
        while let Some(parent_weak) = {
            let borrow = current.borrow();
            borrow.parent.clone()
        } {
            if let Some(parent) = parent_weak.upgrade() {
                current = parent;
                depth += 1;
            } else {
                break;
            }
        }
        depth
    }

    pub fn tree_search(node: &Option<BstNodeLink>, key: i32) -> Option<BstNodeLink> {
        let mut current = node.clone();
        while let Some(node_ref) = current {
            let node_borrow = node_ref.borrow();
            let next = if key == node_borrow.key {
                return Some(node_ref.clone());
            } else if key < node_borrow.key {
                node_borrow.left.clone()
            } else {
                node_borrow.right.clone()
            };
            current = next;
        }
        None
    }
}

impl Clone for BstNode {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            left: self.left.clone(),
            right: self.right.clone(),
            parent: self.parent.clone(),
        }
    }
}
