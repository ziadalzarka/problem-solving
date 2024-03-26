use std::collections::HashMap;

#[derive(Debug)]
struct WordDict {
    hashmap: HashMap<u8, Box<WordDict>>,
    end_of_word: bool,
}

impl WordDict {
    fn new(end_of_word: bool) -> Self {
        WordDict {
            hashmap: HashMap::new(),
            end_of_word,
        }
    }
}

struct WordDictionary {
    word_dict: WordDict,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            word_dict: WordDict::new(false),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut last_hashmap_ref = &mut self.word_dict;

        for (index, &character) in word.as_bytes().iter().enumerate() {
            match last_hashmap_ref.hashmap.contains_key(&character) {
                true => {
                    last_hashmap_ref = last_hashmap_ref
                        .hashmap
                        .get_mut(&character)
                        .unwrap()
                        .as_mut();
                }
                false => {
                    let new_word_dict = Box::new(WordDict::new(index == word.len() - 1));
                    last_hashmap_ref.hashmap.insert(character, new_word_dict);
                    last_hashmap_ref = last_hashmap_ref
                        .hashmap
                        .get_mut(&character)
                        .unwrap()
                        .as_mut();
                }
            }
        }
    }

    fn _search(&self, word_dict: &WordDict, word: &str) -> bool {
        let mut last_word_dict_ref = word_dict;

        for (index, &character) in word.as_bytes().iter().enumerate() {
            if character == b'.' {
                for (_, value) in last_word_dict_ref.hashmap.iter() {
                    if self._search(&value, &word[index + 1..]) == true {
                        return true;
                    }
                }
            }

            match last_word_dict_ref.hashmap.get(&character) {
                Some(map) => {
                    last_word_dict_ref = map.as_ref();
                }
                None => {
                    return false;
                }
            }
        }

        last_word_dict_ref.end_of_word
    }

    fn search(&self, word: String) -> bool {
        self._search(&self.word_dict, word.as_str())
    }
}

fn main() {
    let mut obj = WordDictionary::new();
    obj.add_word(String::from("a"));
    obj.add_word(String::from("ab"));
    dbg!(obj.search(String::from("a")));
    dbg!(obj.search(String::from("a.")));
    dbg!(obj.search(String::from("ab")));
    dbg!(obj.search(String::from(".a")));
    dbg!(obj.search(String::from(".b")));
    dbg!(obj.search(String::from("ab.")));
    dbg!(obj.search(String::from(".")));
    dbg!(obj.search(String::from("..")));
}
