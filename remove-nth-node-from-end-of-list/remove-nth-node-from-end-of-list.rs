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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut fast_pointer = head.as_ref();

    // Unwrap is safe because length of linked list is guaranteed to be > n
    for _ in 0..n {
        fast_pointer = fast_pointer.unwrap().next.as_ref();
    }

    let mut node_index = 0;

    while let Some(node) = fast_pointer {
        node_index += 1;
        fast_pointer = node.next.as_ref();
    }

    let mut slow_pointer = head.as_mut();

    // Unwrap is safe because length of linked list is guaranteed to be > node_index
    for _ in 0..node_index - 1 {
        slow_pointer = slow_pointer.unwrap().next.as_mut();
    }

    // Node before removed node
    let new_previous_pointer = slow_pointer;

    dbg!(new_previous_pointer.as_ref());

    // Unwrap is safe because the node is guaranteed to exist
    let removed_pointer = new_previous_pointer
        .as_ref()
        .unwrap()
        .next
        .as_ref();

    dbg!(removed_pointer);

    // // Must match because removed node might be the last node
    // let new_next_pointer = match removed_pointer.next.as_ref() {
    //     Some(next_pointer) => next_pointer.next.as_ref(),
    //     None => None,
    // };

    // new_previous_pointer.unwrap().next = Some(new_next_pointer.unwrap().to_owned());

    head
}

fn main() {
    // let list = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 3,
    //             next: Some(Box::new(ListNode {
    //                 val: 4,
    //                 next: Some(Box::new(ListNode::new(5))),
    //             })),
    //         })),
    //     })),
    // }));

    let list = Some(Box::new(ListNode { val: 1, next: None }));

    let result = remove_nth_from_end(list, 1);

    dbg!(result);
}
