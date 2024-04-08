use std::collections::{BinaryHeap, HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut counter_hashmap: HashMap<char, i32> = HashMap::new();

        for task in tasks {
            counter_hashmap.insert(task, counter_hashmap.get(&task).unwrap_or(&0) + 1);
        }

        for entry in counter_hashmap.into_values() {
            heap.push(entry);
        }

        let mut time = 0;
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        while heap.len() > 0 || queue.len() > 0 {
            time += 1;

            match queue.front().as_ref() {
                Some((occurances, time_scheduled)) => {
                    if time.eq(time_scheduled) {
                        heap.push(*occurances);
                        queue.pop_front();
                    }
                },
                None => (),
            }

            match heap.pop() {
                Some(occurances) => {
                    if occurances > 1 {
                        queue.push_back((occurances - 1, time + n + 1));
                    }
                },
                None => (),
            }
        }

        time
    }
}

fn main() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    let result = Solution::least_interval(tasks, n);
    println!("Result: {}", result);
}
