use std::collections::LinkedList;

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // Contains the heights we can extend
    let mut stack: LinkedList<(usize, &i32)> = LinkedList::new();
    let mut max_area: i32 = 0;

    let mut index = 0;

    while index < heights.len() {
        let height = &heights[index];

        if stack.is_empty() {
            stack.push_back((index, height))
        } else {
            let mut start_index = index.clone();

            loop {
                let (last_index, last_height) = match stack.back() {
                    Some(value) => value.clone(),
                    None => (0, &0),
                };

                if *height < *last_height {
                    stack.pop_back();

                    start_index = last_index;

                    let area = last_height * (index - last_index) as i32;

                    if area > max_area {
                        max_area = area;
                    }
                } else {
                    break;
                }
            }

            stack.push_back((start_index, height));
        }

        index += 1;
    }

    stack.iter().for_each(|(i, h)| {
        let area = (heights.len() - *i) as i32 * **h;

        if area > max_area {
            max_area = area;
        }
    });

    max_area
}

fn main() {
    println!("{}", largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]));
}
