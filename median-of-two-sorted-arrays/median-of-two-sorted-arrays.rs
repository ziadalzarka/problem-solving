struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let median: f64 = 0.0;
        let total_size = nums1.len() + nums2.len();
        
        let first_left = 0;
        let first_right = nums2.len() / 2;
        let second_left = 0;
        let second_right = total_size - 1 - first_right;

        // Perform checks
        

        median
    }
}

fn main() {
    let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
    dbg!(result);
}
