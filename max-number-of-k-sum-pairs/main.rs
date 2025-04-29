use std::cmp;

struct Solution {}

// Hashmap solution
// use std::collections::HashMap;
// impl Solution {
//     pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
//         let mut frequency: HashMap<i32, i32> = HashMap::new();
//         let mut operations_count = 0;

//         for num in nums {
//             let complement = k - num;

//             if let Some(&freq) = frequency.get(&complement) {
//                 if freq > 0 {
//                     operations_count += 1;
//                     *frequency.entry(complement).or_insert(0) -= 1;
//                     continue;
//                 }
//             }

//             *frequency.entry(num).or_insert(0) += 1;
//         }

//         operations_count
//     }
// }

// Two pointer solution
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        
        nums.sort();

        let mut l = 0;
        let mut r = nums.len() - 1;

        let mut count = 0;

        while l < r {
            let sum = nums.get(l).unwrap() + nums.get(r).unwrap();

            dbg!(sum, l, r, count);

            match sum.cmp(&k) {
                cmp::Ordering::Less => {
                    l += 1;
                },
                cmp::Ordering::Equal => {
                    count += 1;
                    l += 1;
                    r -= 1;
                },
                cmp::Ordering::Greater => {
                    r -= 1;
                },
            }
        }

        count
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations() {
        // assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
        // assert_eq!(Solution::max_operations(vec![1, 3, 3, 3, 4], 6), 1);
        assert_eq!(Solution::max_operations(vec![3, 2, 3, 4, 4], 6), 2);
        assert_eq!(Solution::max_operations(vec![], 5), 0);
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 10), 0);
        assert_eq!(
            Solution::max_operations(vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4], 6),
            2
        );
        assert_eq!(
            Solution::max_operations(
                vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2],
                3
            ),
            4
        );
    }
}
