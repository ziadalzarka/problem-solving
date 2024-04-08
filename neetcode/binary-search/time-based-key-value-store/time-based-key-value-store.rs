use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.map.get_mut(&key) {
            Some(timestamp_map) => {
                match timestamp_map.binary_search_by_key(&timestamp, |(_, time)| *time) {
                    Ok(_) => (),
                    Err(_) => {
                        // Numbers are strictly increasing, no need to keep array sorted or implemented a self balancing tree
                        timestamp_map.push((value, timestamp));
                    }
                }
            }
            None => {
                self.map.insert(key, vec![(value, timestamp)]);
            }
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            Some(timestamp_map) => {
                match timestamp_map.binary_search_by_key(&timestamp, |(_, time)| *time) {
                    Ok(index) => timestamp_map[index].0.clone(),
                    Err(index) => match timestamp_map.get(index - 1) {
                        Some((value, _)) => value.clone(),
                        None => String::from(""),
                    },
                }
            }
            None => String::from(""),
        }
    }
}

fn main() {
    let mut obj = TimeMap::new();

    obj.set(String::from("foo"), String::from("bar"), 1);

    dbg!(obj.get(String::from("foo"), 1));
    dbg!(obj.get(String::from("foo"), 3));

    obj.set(String::from("foo"), String::from("bar2"), 4);

    dbg!(obj.get(String::from("foo"), 4));
    dbg!(obj.get(String::from("foo"), 5));
}
