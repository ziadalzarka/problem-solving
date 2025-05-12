struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    fn heapify_subtree(array: &mut Vec<i32>, root: usize) {
        let l = root * 2;
        let r = root * 2 + 1;

        let mut min;

        if array.len() > l {
            min = l;
        } else {
            return;
        }

        if array.len() > r && array[r] < array[min] {
            min = r;
        }

        if array[root] > array[min] {
            // Swap
            array[root] = array[root] ^ array[min];
            array[min] = array[root] ^ array[min];
            array[root] = array[root] ^ array[min];

            // Heapify new subtree
            MinHeap::heapify_subtree(array, min);
        }
    }

    fn from(mut array: Vec<i32>) -> MinHeap {
        if array.len() <= 0 {
            return MinHeap { heap: vec![0] };
        }

        array.push(array[0]);

        let internal_nodes_end_index = array.len() / 2;

        // Iterate backwards starting with parent nodes of leaf nodes
        // Leaf nodes are skipped because they are a valid heap of 1 node
        for i in (1..internal_nodes_end_index).rev() {
            // Check if sub-tree is a valid heap and perform swaps if not
            MinHeap::heapify_subtree(&mut array, i);
        }

        MinHeap { heap: array }
    }

    fn peek(self) -> Option<i32> {
        self.heap.get(1).map(|x| *x)
    }
}

struct Solution {}

impl Solution {
    pub fn min(numbers: Vec<i32>) -> i32 {
        if numbers.len() <= 0 {
            return 0;
        }

        MinHeap::from(numbers).peek().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min(vec![1]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min(vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::min(vec![-5, -2, 0, 3, 7]), -5);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::min(vec![10, 5, 20, 30, 15]), 5);
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::min(vec![100, 200, 300, 0, 400]), 0);
    }
}
