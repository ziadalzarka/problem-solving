struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums_a: Vec<i32>, nums_b: Vec<i32>) -> f64 {
        if nums_a.len() == 0 {
            let half = nums_b.len() / 2;
            match nums_b.len() % 2 {
                0 => {
                    return (nums_b[half - 1] + nums_b[half]) as f64 / 2.0;
                }
                _ => {
                    return nums_b[half] as f64;
                }
            }
        }

        if nums_b.len() == 0 {
            let half = nums_a.len() / 2;
            match nums_a.len() % 2 {
                0 => {
                    return (nums_a[half - 1] + nums_a[half]) as f64 / 2.0;
                }
                _ => {
                    return nums_a[half] as f64;
                }
            }
        }

        let total_size = nums_a.len() + nums_b.len();
        let half = total_size / 2;
        let is_odd = (total_size % 2) > 0;

        let mut left = 0;
        let mut right = nums_a.len() - 1;

        while left <= right {
            let middle = (left + right) / 2;

            let partition_a_end_index = middle;
            let partition_b_end_index: Option<usize> = {
                if half - (partition_a_end_index + 1) == 0 {
                    None
                } else {
                    Some(half - partition_a_end_index - 2)
                }
            };

            match partition_b_end_index {
                Some(partition_b_end_index) => {
                    if nums_a[partition_a_end_index]
                        > *nums_b.get(partition_b_end_index + 1).unwrap_or(&i32::MAX)
                    {
                        right = middle - 1;
                    } else if nums_b[partition_b_end_index]
                        > *nums_a.get(partition_a_end_index + 1).unwrap_or(&i32::MAX)
                    {
                        left = middle + 1;
                    } else {
                        let min_right_partition = nums_a[partition_a_end_index + 1]
                            .min(nums_b[partition_b_end_index + 1])
                            as f64;
                        if is_odd {
                            return min_right_partition;
                        } else {
                            let max_left_partition = nums_a[partition_a_end_index]
                                .max(nums_b[partition_b_end_index])
                                as f64;
                            return (max_left_partition + min_right_partition) / 2.0;
                        }
                    }
                }
                None => {
                    let min_right_partition = *nums_a
                        .get(partition_a_end_index + 1)
                        .unwrap_or(&i32::MAX)
                        .min(&nums_b[0]) as f64;
                    if is_odd {
                        return min_right_partition;
                    } else {
                        let max_left_partition = nums_a[partition_a_end_index] as f64;
                        return (max_left_partition + min_right_partition) / 2.0;
                    }
                }
            }
        }

        0.0
    }
}

fn main() {
    let result = Solution::find_median_sorted_arrays(vec![3], vec![-2, -1]);
    dbg!(result);
}
