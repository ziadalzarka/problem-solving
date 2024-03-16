fn car_fleet(target: i32, positions: Vec<i32>, speeds: Vec<i32>) -> i32 {
    let mut stack: Vec<f32> = vec![];

    let mut sorted: Vec<(i32, i32)> = positions
        .iter()
        .zip(speeds.iter())
        .map(|(&position, &speed)| (position, speed))
        .collect();

    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut index = sorted.len() - 1;

    loop {
        let position = sorted[index].0;
        let speed = sorted[index].1;

        let will_arrive_in: f32 = (target as f32 - position as f32) / speed as f32;

        loop {
            match stack.last() {
                Some(next_car_will_arrive_in) => {
                    if *next_car_will_arrive_in < will_arrive_in {
                        stack.push(will_arrive_in);
                    } else {
                        break;
                    }
                }
                // Last car
                None => stack.push(will_arrive_in),
            }
        }

        if index == 0 {
            break;
        }

        index -= 1;
    }

    stack.len() as i32
}

fn main() {
    println!(
        "{}",
        car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3])
    );
}
