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

// [1], 1
// [1, 2, 3, 4, 5] 2

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut fast_pointer = head.as_ref();

    for _ in 0..n {
        fast_pointer = fast_pointer.unwrap().next.as_ref();
    }

    let mut node_index = 0;

    while let Some(node) = fast_pointer {
        node_index += 1;
        fast_pointer = node.next.as_ref();
    }

    let mut fast_pointer = head.as_mut();
    let mut slow_pointer = head.as_mut();

    for _ in 0..node_index - 1 {
        slow_pointer = slow_pointer.unwrap().next.as_mut();
        fast_pointer = fast_pointer.unwrap().next.as_mut();
    }

    // Target node
    fast_pointer = fast_pointer.unwrap().next.as_mut();

    // Next node
    let next_node = fast_pointer.unwrap().next.take();

    slow_pointer.unwrap().next = next_node;

    head
}

fn main() {
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    }));

    // let list = Some(Box::new(ListNode { val: 1, next: None }));

    let result = remove_nth_from_end(list, 2);

    dbg!(result);
}
