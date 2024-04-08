struct Solution {}

impl Solution {
    pub fn merge(nums_a: &mut Vec<i32>, nums_a_len: i32, nums_b: &mut Vec<i32>, nums_b_len: i32) {
        let mut a_right = nums_a_len - 1;
        let mut b_right = nums_b_len - 1;

        let mut pointer = nums_a.len() - 1;

        while a_right >= 0 || b_right >= 0 {
            match nums_a.get(a_right as usize).unwrap_or(&i32::MIN).cmp(&nums_b.get(b_right as usize).unwrap_or(&i32::MIN)) {
                std::cmp::Ordering::Less => {
                    nums_a[pointer] = nums_b[b_right as usize];
                    b_right -= 1;
                    pointer -= 1;
                },
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    nums_a[pointer] = nums_a[a_right as usize];
                    a_right -= 1;
                    if pointer == 0 {
                        break;
                    }
                    pointer -= 1;
                },
            }
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 7, 8, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    dbg!(nums1);
}
