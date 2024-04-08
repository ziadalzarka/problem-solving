use std::cell::RefCell;
use std::rc::Rc;

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

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        String::from("")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let obj = Codec::new();
    let data: String = obj.serialize(root);

    dbg!(&data);

    let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);

    dbg!(&ans);
}
