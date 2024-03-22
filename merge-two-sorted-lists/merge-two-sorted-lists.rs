use std::cmp::Ordering;

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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = (list1, list2);
    let mut sorted_head: Option<Box<ListNode>> = None;
    let mut sorted_tail: Option<&mut Box<ListNode>> = None;

    while let (Some(mut node1), Some(mut node2)) = head {
        let next_node;

        match node1.val.cmp(&node2.val) {
            Ordering::Less | Ordering::Equal => {
                head = (node1.next.take(), Some(node2));
                next_node = node1;
            }
            Ordering::Greater => {
                head = (Some(node1), node2.next.take());
                next_node = node2;
            }
        }

        match sorted_tail {
            None => {
                sorted_head = Some(next_node);
                sorted_tail = sorted_head.as_mut();
            }
            Some(tail) => {
                tail.next = Some(next_node);
                sorted_tail = tail.next.as_mut();
            }
        }
    }

    if let (Some(next_node), None) = head {
        match sorted_tail {
            None => {
                sorted_head = Some(next_node);
                sorted_tail = sorted_head.as_mut();
            }
            Some(tail) => {
                tail.next = Some(next_node);
                sorted_tail = tail.next.as_mut();
            }
        }

        head = (None, None);
    }

    if let (None, Some(next_node)) = head {
        match sorted_tail {
            None => {
                sorted_head = Some(next_node);
            }
            Some(tail) => {
                tail.next = Some(next_node);
            }
        }
    }

    sorted_head
}

fn main() {
    let mut root1 = Box::new(ListNode::new(1));
    let mut r1v2 = Box::new(ListNode::new(2));
    let r1v3 = Box::new(ListNode::new(4));

    r1v2.next = Some(r1v3);
    root1.next = Some(r1v2);

    let mut root2 = Box::new(ListNode::new(1));
    let mut r2v2 = Box::new(ListNode::new(3));
    let r2v3 = Box::new(ListNode::new(4));

    r2v2.next = Some(r2v3);
    root2.next = Some(r2v2);

    dbg!(merge_two_lists(Some(root1), Some(root2)));
}
