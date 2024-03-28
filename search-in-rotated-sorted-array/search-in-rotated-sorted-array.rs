use std::cmp::Ordering;

fn offset_index(size: usize, offset: usize, target: usize) -> usize {
    let end_index = size - 1;
    let initial_offset = target + offset;

    if initial_offset > end_index {
        return initial_offset - end_index - 1;
    }

    initial_offset
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let size = nums.len();

    let mut left = 0;
    let mut right = size - 1;
    let mut offset = 0;

    while left < right {
        let middle = (left + right) / 2;

        if right - left == 1 {
            offset = match nums[right].cmp(&nums[left]) {
                Ordering::Less | Ordering::Equal => right,
                Ordering::Greater => left,
            };
            break;
        }

        match nums[middle].cmp(&nums[right]) {
            Ordering::Less => {
                right = middle;
            }
            Ordering::Equal => {
                offset = middle;
                break;
            }
            Ordering::Greater => {
                left = middle;
            }
        }
    }

    let mut left = 0;
    let mut right = size - 1;

    while left <= right {
        let middle = (left + right) / 2;
        let middle_offset = offset_index(size, offset, middle);

        match nums[middle_offset].cmp(&target) {
            Ordering::Less => {
                left = middle + 1;
            }
            Ordering::Equal => {
                return middle_offset as i32;
            }
            Ordering::Greater => {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            }
        }
    }

    -1
}

fn main() {
    dbg!(search(vec![1], 0));
}
