struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums_a: Vec<i32>, nums_b: Vec<i32>) -> f64 {

        let mut smaller_arr = &nums_a;
        let mut larger_arr = &nums_b;

        if nums_a.len() > nums_b.len() {
            smaller_arr = &nums_b;
            larger_arr = &nums_a;
        }

        if larger_arr.len() == 0 {
            let half = smaller_arr.len() / 2;
            match smaller_arr.len() % 2 {
                0 => {
                    return (smaller_arr[half - 1] + smaller_arr[half]) as f64 / 2.0;
                }
                _ => {
                    return smaller_arr[half] as f64;
                }
            }
        }

        if smaller_arr.len() == 0 {
            let half = larger_arr.len() / 2;
            match larger_arr.len() % 2 {
                0 => {
                    return (larger_arr[half - 1] + larger_arr[half]) as f64 / 2.0;
                }
                _ => {
                    return larger_arr[half] as f64;
                }
            }
        }

        let total_size = larger_arr.len() + smaller_arr.len();
        let half = total_size / 2;
        let is_odd = (total_size % 2) > 0;

        let mut left = 0;
        let mut right = larger_arr.len() - 1;

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
                    if larger_arr[partition_a_end_index]
                        > *smaller_arr.get(partition_b_end_index + 1).unwrap_or(&i32::MAX)
                    {
                        right = middle - 1;
                    } else if smaller_arr[partition_b_end_index]
                        > *larger_arr.get(partition_a_end_index + 1).unwrap_or(&i32::MAX)
                    {
                        left = middle + 1;
                    } else {
                        let min_right_partition = larger_arr[partition_a_end_index + 1]
                            .min(smaller_arr[partition_b_end_index + 1])
                            as f64;
                        if is_odd {
                            return min_right_partition;
                        } else {
                            let max_left_partition = larger_arr[partition_a_end_index]
                                .max(smaller_arr[partition_b_end_index])
                                as f64;
                            return (max_left_partition + min_right_partition) / 2.0;
                        }
                    }
                }
                None => {
                    let min_right_partition = *larger_arr
                        .get(partition_a_end_index + 1)
                        .unwrap_or(&i32::MAX)
                        .min(&smaller_arr[0]) as f64;
                    if is_odd {
                        return min_right_partition;
                    } else {
                        let max_left_partition = larger_arr[partition_a_end_index] as f64;
                        return (max_left_partition + min_right_partition) / 2.0;
                    }
                }
            }
        }

        0.0
    }
}

fn main() {
    let result = Solution::find_median_sorted_arrays(vec![-2, -1], vec![3, 4, 5]);
    dbg!(result);
}
