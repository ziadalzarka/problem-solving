use std::collections::BinaryHeap;

struct Solution {}

impl Solution {

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let heap = BinaryHeap::from(nums).into_vec();

        if k == 1 {
            return heap[0];
        }

        for i in 2..k {
            let r = i * 2;
            let l = i * 2 + 1;

            return heap[r as usize].min(heap[l as usize]);
        }

        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 1), 6);
        // assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        // assert_eq!(
        //     Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
        //     4
        // );
    }
}
