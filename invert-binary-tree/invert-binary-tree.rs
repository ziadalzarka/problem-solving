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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root.as_ref() {
            Some(node) => {
                let mut node = node.borrow_mut();

                let swapped_left = Solution::invert_tree(node.left.take());
                let swapped_right = Solution::invert_tree(node.right.take());

                node.left = swapped_right;
                node.right = swapped_left;
            }
            None => (),
        }

        root
    }
}

fn main() {
    // Create the tree nodes
    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node9 = Rc::new(RefCell::new(TreeNode::new(9)));

    // Build the tree structure
    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node7.clone());
    node2.borrow_mut().left = Some(node1.clone());
    node2.borrow_mut().right = Some(node3.clone());
    node7.borrow_mut().left = Some(node6.clone());
    node7.borrow_mut().right = Some(node9.clone());

    // Invert the binary tree
    let inverted_tree = Solution::invert_tree(Some(root));

    dbg!(inverted_tree);
}
