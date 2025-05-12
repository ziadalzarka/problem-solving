struct MaxHeap {
    heap: Vec<i32>,
}

impl MaxHeap {
    fn new() -> MaxHeap {
        MaxHeap { heap: vec![0] }
    }

    fn from(mut array: Vec<i32>) -> MaxHeap {
        if array.len() <= 0 {
            return MaxHeap::new();
        }

        array.push(array[0]);

        let internal_nodes_end_index = array.len() / 2;

        // Iterate backwards starting with parent nodes of leaf nodes
        // Leaf nodes are skipped because they are a valid heap of 1 node
        for i in (1..internal_nodes_end_index).rev() {
            // Check if sub-tree is a valid heap and perform swaps if not
            MaxHeap::heapify_subtree(&mut array, i);
        }

        MaxHeap { heap: array }
    }

    fn heapify_subtree(array: &mut Vec<i32>, root: usize) {
        let l = root * 2;
        let r = root * 2 + 1;

        let mut max;

        if array.len() > l {
            max = l;
        } else {
            return;
        }

        if array.len() > r && array[r] > array[max] {
            max = r;
        }

        if array[root] < array[max] {
            // Swap
            array[root] = array[root] ^ array[max];
            array[max] = array[root] ^ array[max];
            array[root] = array[root] ^ array[max];

            // Heapify new subtree
            MaxHeap::heapify_subtree(array, max);
        }
    }

    fn peek(&self) -> Option<i32> {
        self.heap.get(1).map(|x| *x)
    }

    fn pop(&mut self) -> Option<i32> {
        if self.heap.len() == 1 {
            return None;
        }

        let popped = self.peek();

        if let Some(last) = self.heap.pop() {
            if self.heap.len() > 1 {
                self.heap[1] = last;
                MaxHeap::heapify_subtree(&mut self.heap, 1);
            }
        }

        popped
    }
}

struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = MaxHeap::from(nums);

        let mut popped = heap.peek();

        for _ in 1..=k {
            popped = heap.pop();
        }

        popped.unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 1), 6);
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            3
        );
    }
}
