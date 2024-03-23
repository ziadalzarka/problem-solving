use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut map: HashMap<u8, usize> = HashMap::new();

    let mut start_index = 0;
    let mut current_length = 0;

    for (index, character) in s.as_bytes().iter().enumerate() {
        match map.get(&character) {
            Some(&duplicated_character_index) => {
                for i in start_index..duplicated_character_index + 1 {
                    map.remove(&s.as_bytes()[i]);
                }
                map.insert(*character, index);
                start_index = duplicated_character_index + 1;
                current_length = (index - start_index) + 1;
            }
            None => {
                map.insert(*character, index);
                current_length += 1;
            }
        }

        if current_length > max_length {
            max_length = current_length;
        }
    }

    max_length as i32
}

fn main() {
    dbg!(length_of_longest_substring(String::from("dvdf")));
}
