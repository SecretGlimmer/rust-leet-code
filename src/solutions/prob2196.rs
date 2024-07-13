pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut root_node: Option<Rc<RefCell<TreeNode>>> = None;

        for desc in &descriptions {
            let (parent_i, child_i, is_left) = (desc[0], desc[1], desc[2]);

            let parent_node = nodes.get(&parent_i).get_or_insert_with(|| &Rc::new(RefCell::new(TreeNode::new(parent_i))));
            let child_node = nodes.get(&parent_i).get_or_insert_with(|| &Rc::new(RefCell::new(TreeNode::new(child_i))));

            if is_left == 1 {
                parent_node.borrow_mut().left = Some(child_node.clone());
            } else {
                parent_node.borrow_mut().right = Some(child_node.clone());
            }
        }

        root_node
    }
}
