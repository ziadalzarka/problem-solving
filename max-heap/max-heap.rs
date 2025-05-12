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
}

struct Solution {}

impl Solution {
    pub fn max(numbers: Vec<i32>) -> i32 {
        if numbers.len() <= 0 {
            return 0;
        }

        MaxHeap::from(numbers).peek().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_case1() {
        assert_eq!(MaxHeap::from(vec![2, 7, 4, 1, 8, 1]).peek().unwrap(), 8);
    }

    #[test]
    fn max_case2() {
        assert_eq!(MaxHeap::from(vec![1]).peek().unwrap(), 1);
    }

    #[test]
    fn max_case3() {
        assert_eq!(MaxHeap::from(vec![5, 4, 3, 2, 1]).peek().unwrap(), 5);
    }

    #[test]
    fn max_case4() {
        assert_eq!(MaxHeap::from(vec![-5, -2, 0, 3, 7]).peek().unwrap(), 7);
    }

    #[test]
    fn max_case5() {
        assert_eq!(MaxHeap::from(vec![10, 5, 20, 30, 15]).peek().unwrap(), 30);
    }

    #[test]
    fn max_case6() {
        assert_eq!(
            MaxHeap::from(vec![100, 200, 300, 0, 400]).peek().unwrap(),
            400
        );
    }

    #[test]
    fn push_case1() {
        let mut heap = MaxHeap::new();

        heap.push(8);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(1);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(2);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(3);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(4);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(5);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(6);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(7);
        assert_eq!(heap.peek().unwrap(), 8);

        heap.push(12);
        assert_eq!(heap.peek().unwrap(), 12);

        heap.push(10);
        assert_eq!(heap.peek().unwrap(), 12);

        heap.push(20);
        assert_eq!(heap.peek().unwrap(), 20);

        heap.push(30);
        assert_eq!(heap.peek().unwrap(), 30);

        heap.push(100);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(50);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(40);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(30);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(40);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(100);
        assert_eq!(heap.peek().unwrap(), 100);

        heap.push(101);
        assert_eq!(heap.peek().unwrap(), 101);
    }

    #[test]
    fn pop_case1() {
        let mut heap = MaxHeap::from(vec![1, 2, 3, 4, 100, 200, 500]);

        assert_eq!(heap.pop().unwrap(), 500);
        assert_eq!(heap.pop().unwrap(), 200);
        assert_eq!(heap.pop().unwrap(), 100);
        assert_eq!(heap.pop().unwrap(), 4);
        assert_eq!(heap.pop().unwrap(), 3);
        assert_eq!(heap.pop().unwrap(), 2);
        assert_eq!(heap.pop().unwrap(), 1);
        assert_eq!(heap.pop(), None);
    }
}
