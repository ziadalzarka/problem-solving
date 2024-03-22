fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let &max = piles.iter().max().unwrap_or(&0);

    let mut left = 1;
    let mut right = max;
    let mut middle;
    let mut k = max;

    while left <= right {
        middle = (right + left) / 2;

        let hours: i64 = piles.iter().fold(0, |acc: i64, bananas_count: &i32| {
            acc + ((*bananas_count as i64 + (middle as i64 - 1) as i64) / middle as i64)
        });

        println!("{left} {middle} {right} in {hours} hours");

        match (hours).cmp(&(h as i64)) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                if middle < k {
                    k = middle
                }

                if middle == 0 {
                    break;
                }

                right = middle - 1;
            }
            std::cmp::Ordering::Greater => {
                left = middle + 1;
            }
        }

        println!("k is {k}");
    }

    k
}

fn main() {
    println!(
        "{}",
        min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000)
    );
}
