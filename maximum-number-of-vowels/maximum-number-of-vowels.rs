// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/description/?envType=study-plan-v2&envId=leetcode-75

struct Solution {}

impl Solution {
    pub fn is_vowel_letter(b: u8) -> bool {
        matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;

        let mut l = 0;
        let mut r = k - 1;

        let bytes: &[u8] = s.as_bytes();

        let mut current_count = 0;

        for c in &bytes[0..k] {
            if Solution::is_vowel_letter(*c) {
                current_count += 1;
            }
        }

        let mut max_count = current_count;

        while r < s.len() - 1 {
            let left = bytes[l];

            if Solution::is_vowel_letter(left) {
                current_count -= 1;
            }

            l += 1;
            r += 1;

            let next = bytes[r];

            if Solution::is_vowel_letter(next) {
                current_count += 1;
            }

            if current_count > max_count {
                max_count = current_count;
            }
        }

        max_count
    }
}

fn main() {
    dbg!(Solution::max_vowels(String::from("abciihdeiaf"), 3));
}
