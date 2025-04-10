use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

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
            right: None,
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();

                let left_max_depth = match node.left.as_ref() {
                    Some(left) => {
                        Solution::max_depth(Some(left.clone()))
                    },
                    None => 0
                };

                let right_max_depth = match node.right.as_ref() {
                    Some(right) => {
                        Solution::max_depth(Some(right.clone()))
                    },
                    None => 0
                };

                1 + i32::max(left_max_depth, right_max_depth)
            },
            None => 0
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let node2 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    let node4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    root.as_ref().unwrap().borrow_mut().left = node1.clone();
    root.as_ref().unwrap().borrow_mut().right = node2.clone();
    node2.as_ref().unwrap().borrow_mut().left = node3.clone();
    node2.as_ref().unwrap().borrow_mut().right = node4.clone();

    let result = Solution::max_depth(root);
    dbg!(result);
}
