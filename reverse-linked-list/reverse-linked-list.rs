// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev_node: Option<Box<ListNode>> = None;

    while let Some(mut current_node) = head {
        let next = current_node.next.take();

        current_node.next = prev_node;

        prev_node = Some(current_node);

        head = next;
    }

    prev_node
}

fn main() {
    let mut root = Box::new(ListNode::new(1));
    let mut val2 = Box::new(ListNode::new(2));
    let mut val3 = Box::new(ListNode::new(3));
    let mut val4 = Box::new(ListNode::new(4));
    let val5 = Box::new(ListNode::new(5));

    val4.next = Some(val5);
    val3.next = Some(val4);
    val2.next = Some(val3);
    root.next = Some(val2);

    dbg!(reverse_list(Some(root)));
}
