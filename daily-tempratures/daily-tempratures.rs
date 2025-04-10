fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<(usize, i32)> = vec![];

    let mut index = 0;
    while index < temperatures.len() {
        let temperature = temperatures[index];

        if index == 0 {
            stack.push((0, temperature));
            index += 1;
            continue;
        }

        loop {
            match stack.last() {
                Some((last_index, last_temperature)) => {
                    if temperature > *last_temperature {
                        result[*last_index] = (index - *last_index) as i32;
                        stack.pop();
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            };
        }

        stack.push((index, temperature));

        index += 1
    }

    result
}

fn main() {
    let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
    let joined = result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", joined);
}
