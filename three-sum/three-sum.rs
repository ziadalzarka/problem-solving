use serde_json;

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    let mut sorted = nums.clone();
    sorted.sort();

    for (index, num) in sorted.iter().enumerate() {
        println!("In iteration {}, {}", index, num);
        if index != 0 && sorted[index - 1] == *num {
            continue;
        }

        if *num > 0 {
            break;
        }

        let mut left = index + 1;
        let mut right = sorted.len() - 1;

        while left < right {
            let three_sum = num + sorted[left] + sorted[right];

            match three_sum.cmp(&0) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => {
                    result.push(vec![sorted[index], sorted[left], sorted[right]]);
                    left += 1;
                    while left < right && sorted[left] == sorted[left - 1] {
                        left += 1;
                    }
                }
            }
        }
    }

    result
}

fn main() {
    println!(
        "{}",
        serde_json::to_string(&three_sum(vec![-1, 0, 1, 2, -1, -4])).unwrap()
    )
}
