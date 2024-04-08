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
    pub fn diameter(root: Option<Rc<RefCell<TreeNode>>>, maximum_diameter: &mut i32) -> i32 {
        match root.as_ref() {
            Some(node) => {
                let node = node.borrow();

                let left_depth = match node.left.as_ref() {
                    Some(left) => Solution::diameter(Some(left.clone()), maximum_diameter),
                    None => 0,
                };

                let right_depth = match node.right.as_ref() {
                    Some(right) => Solution::diameter(Some(right.clone()), maximum_diameter),
                    None => 0,
                };

                let left_depth = left_depth + 1;
                let right_depth = right_depth + 1;

                let diameter = left_depth + right_depth - 2;

                if diameter > *maximum_diameter {
                    *maximum_diameter = diameter;
                }

                i32::max(left_depth, right_depth)
            }
            None => 1,
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root.as_ref() {
            Some(node) => {
                let mut diameter = 0;
                Solution::diameter(Some(node.clone()), &mut diameter);
                diameter
            }
            None => 0,
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let result = Solution::diameter_of_binary_tree(root);
    println!("Diameter of binary tree: {}", result);
}
