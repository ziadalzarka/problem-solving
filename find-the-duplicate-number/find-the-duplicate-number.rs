pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32 - 1;
    let mut sum = (n * (1 + n)) / 2;

    for num in nums {
        sum -= num;
    }

    -1 * sum
}

fn main() {
    dbg!(find_duplicate(vec![1, 3, 4, 2, 2]));
}
