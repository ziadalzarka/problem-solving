use std::cmp::Ordering;

fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let min = nums[0];

    while left < right {
        let middle = (left + right) / 2;

        if right - left == 1 {
            return std::cmp::min(nums[right], nums[left]);
        }

        match nums[middle].cmp(&nums[right]) {
            Ordering::Less => {
                right = middle;
            }
            Ordering::Equal => {
                return nums[middle];
            }
            Ordering::Greater => {
                left = middle;
            }
        }
    }

    min
}

fn main() {
    println!("{}", find_min(vec![2, 3, 1]));
}
