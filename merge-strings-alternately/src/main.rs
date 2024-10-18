struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        result.reserve(word1.len() + word2.len());

        let mut word1_enumerator = word1.chars();
        let mut word2_enumerator = word2.chars();

        while let Some(c1) = word1_enumerator.next() {
            result.push(c1);

            if let Some(c2) = word2_enumerator.next() {
                result.push(c2);
            }
        }

        while let Some(c2) = word2_enumerator.next() {
            result.push(c2);
        }

        result
    }
}

fn main() {
    let result = Solution::merge_alternately("abc456789".to_string(), "pqr123".to_string());
    dbg!(result);
}
