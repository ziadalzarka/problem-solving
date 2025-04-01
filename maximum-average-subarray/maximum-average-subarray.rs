struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let length = nums.len();

        if length < (k as usize) {
            return 0f64;
        }

        let mut l: usize = 0;
        let mut r: usize = (k as usize) - 1;

        let mut sum: i32 = nums[l..r + 1].iter().sum();

        let mut max_average = sum as f64 / k as f64;

        dbg!(sum);
        dbg!(max_average);

        while r < length - 1 {
            l += 1;
            r += 1;

            sum -= nums.get(l - 1).unwrap();
            sum += nums.get(r).unwrap();

            let average = sum as f64 / k as f64;

            dbg!(sum);
            dbg!(average);

            if average > max_average {
                max_average = average;
            }
        }

        max_average
    }
}

fn main() {
    // dbg!(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
    // dbg!(Solution::find_max_average(vec![5], 1));
    dbg!(Solution::find_max_average(vec![4,0,4,3,3], 5));
}
