use std::collections::VecDeque;

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

pub fn reorder_list(root: &mut Option<Box<ListNode>>) {
    let mut head = match root {
        Some(root_node) => root_node.next.take(),
        None => return,
    };

    let mut nodes: VecDeque<Option<Box<ListNode>>> = VecDeque::new();

    while let Some(mut node) = head {
        let next_node = node.next.take();
        nodes.push_back(Some(node));
        head = next_node;
    }

    fn populate_list(
        root: &mut Option<Box<ListNode>>,
        nodes: &mut VecDeque<Option<Box<ListNode>>>,
        left: bool,
    ) {
        match root {
            Some(node) => {
                if left {
                    if let Some(next_node) = nodes.pop_front() {
                        node.next = next_node;
                        populate_list(&mut node.next, nodes, !left);
                    }
                } else {
                    if let Some(next_node) = nodes.pop_back() {
                        node.next = next_node;
                        populate_list(&mut node.next, nodes, !left);
                    }
                }
            }
            None => (),
        }
    }

    populate_list(root, &mut nodes, false);
}

fn main() {
    let mut list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        })),
    }));

    reorder_list(&mut list);
    dbg!(list);
}
