use std::cmp::Ordering;

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let target_index = -1;

    let mut left = 0;
    let mut right = nums.len() - 1;
    let pivot_index;

    while left <= right {
        let middle = (left + right) / 2;

        match nums[middle].cmp(&nums[right]) {
            Ordering::Less => {
                right = middle - 1;
            }
            Ordering::Equal => {
                pivot_index = middle;
                break;
            }
            Ordering::Greater => {
                left = middle + 1;
            }
        }
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        
    }

    target_index
}

fn main() {
    dbg!(search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}
