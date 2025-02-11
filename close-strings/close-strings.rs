struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut word1_hashmap = [0; 26];
        let mut word2_hashmap = [0; 26];

        for char in word1.chars() {
            let index = (char as usize) - 97;
            word1_hashmap[index] += 1;
        }

        for char in word2.chars() {
            let index = (char as usize) - 97;
            word2_hashmap[index] += 1;
        }

        for i in 0..26 {
            if (word1_hashmap[i] == 0 && word2_hashmap[i] != 0)
                || (word1_hashmap[i] != 0 && word2_hashmap[i] == 0)
            {
                return false;
            }
        }

        word1_hashmap.sort();
        word2_hashmap.sort();

        word1_hashmap == word2_hashmap
    }
}

fn main() {
    dbg!(Solution::close_strings(
        String::from("abc"),
        String::from("acb")
    ));
    dbg!(Solution::close_strings(
        String::from("a"),
        String::from("aa")
    ));
    dbg!(Solution::close_strings(
        String::from("cabbba"),
        String::from("abbccc")
    ));
    dbg!(Solution::close_strings(
        String::from("uau"),
        String::from("ssx")
    ));
}
