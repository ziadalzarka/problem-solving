use std::cmp::Ordering;

fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut min = nums[0];

    if nums[right] > nums[left] {
        return min;
    }

    while left < right {
        let middle = (left + right) / 2;

        println!("P: {left} {middle} {right}");
        println!("N: {} {} {}", nums[left], nums[middle], nums[right]);

        match (
            nums[middle].cmp(&nums[left]),
            nums[middle].cmp(&nums[right]),
        ) {
            (Ordering::Greater, Ordering::Less) => return nums[left],
            (Ordering::Less, Ordering::Less) => right = middle,
            (Ordering::Less, Ordering::Equal) => return nums[right],
            (Ordering::Equal, Ordering::Equal) => return nums[middle],
            (Ordering::Equal, Ordering::Greater) => return nums[right],
            (Ordering::Equal, Ordering::Less) => return nums[left],
            (Ordering::Greater, Ordering::Greater) => left = middle,
            _ => break,
        }
    }

    min
}

fn main() {
    println!("{}", find_min(vec![4, 5, 6, 7, 0, 1, 2]));
}
