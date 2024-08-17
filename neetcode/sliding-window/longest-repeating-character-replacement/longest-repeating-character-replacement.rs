use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn count_all(map: &HashMap<char, i32>) -> i32 {
        map.values().into_iter().fold(0, |init, count| init + count)
    }

    fn find_most_frequent(map: &HashMap<char, i32>) -> char {
        *map.into_iter().max_by_key(|(_, value)| **value).unwrap().0
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut left_chars = s.chars();
        let mut right_chars = s.chars();

        let mut most_frequent_character: char = s.chars().next().unwrap();

        let mut counts: HashMap<char, i32> = HashMap::new();
        counts.insert(most_frequent_character, 1);

        loop {
            // Check if window is valid
            let all_count = Solution::count_all(&counts);
            let most_frequent_count = counts.get(&most_frequent_character).unwrap().clone();
            let other_counts = all_count - most_frequent_count;

            let is_window_valid = (most_frequent_count - other_counts) <= k;

            dbg!(
                all_count,
                most_frequent_character,
                most_frequent_count,
                other_counts,
                is_window_valid
            );

            if is_window_valid {
                let char = right_chars.next();

                if let None = char {
                    break;
                }

                let char = char.unwrap();

                let previous_count = counts.remove(&char).unwrap_or(0);
                let new_count = previous_count + 1;

                counts.insert(char, new_count);

                if char != most_frequent_character && new_count > most_frequent_count {
                    most_frequent_character = char;
                }

                dbg!(&char, &most_frequent_character, &most_frequent_count);
            } else {
                let char = left_chars.next().unwrap();

                let previous_count = counts.remove(&char).unwrap();
                let new_count = previous_count - 1;

                counts.insert(char, new_count);

                most_frequent_character = Solution::find_most_frequent(&counts);
            }
        }

        0
    }
}

fn main() {
    dbg!(Solution::character_replacement("XYYX".to_string(), 1));
}
