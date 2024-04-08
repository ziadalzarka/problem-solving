fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;

    while left <= right {
        let middle: i32 = (right + left) / 2;
        let num: &i32 = nums.get(middle as usize).unwrap();

        if target == *num {
            return middle;
        } else if target > *num {
            left = middle + 1;
        } else if target < *num {
            right = middle - 1;
        }
    }

    return -1;
}

fn main() {
    println!("{}", search(vec![1, 2, 3, 4, 5, 6], 4));
}
