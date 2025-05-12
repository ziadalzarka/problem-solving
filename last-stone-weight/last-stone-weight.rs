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

    fn push(&mut self, value: i32) {
        self.heap.push(value);
        self.perpetuate_up(self.heap.len() - 1);
    }

    fn perpetuate_up(&mut self, i: usize) {
        let parent = i / 2;

        if parent < 1 {
            return;
        }

        if self.heap[i] > self.heap[parent] {
            // Swap if child is bigger
            self.heap[parent] = self.heap[parent] ^ self.heap[i];
            self.heap[i] = self.heap[parent] ^ self.heap[i];
            self.heap[parent] = self.heap[parent] ^ self.heap[i];

            // Continue up
            self.perpetuate_up(parent);
        }
    }

    fn len(&self) -> usize {
        self.heap.len() - 1
    }
}

struct Solution {}

impl Solution {
    pub fn last_stone_weight(numbers: Vec<i32>) -> i32 {
        if numbers.len() <= 0 {
            return 0;
        }

        let mut heap = MaxHeap::from(numbers);

        loop {
            if heap.len() == 0 {
                return 0;
            }

            if heap.len() == 1 {
                return heap.peek().unwrap();
            }

            let stone1 = heap.pop().unwrap();
            let stone2 = heap.pop().unwrap();

            if stone1 != stone2 {
                heap.push((stone2 - stone1).abs());
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::last_stone_weight(vec![1, 3]), 2);
    }
}
