use std::cmp::min;

pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_as_chars = word1.chars().collect::<Vec<_>>();
        let word2_as_chars = word2.chars().collect::<Vec<_>>();
        let min_len = min(word1_as_chars.len(), word2_as_chars.len());
        let mut res = String::new();
        for i in 0..min_len {
            res.push_str(&format!("{}{}", word1_as_chars[i], word2_as_chars[i]));
        }
        if min_len == word1_as_chars.len() {
            word2_as_chars[min_len..]
                .iter()
                .for_each(|char| res.push(*char));
        } else {
            word1_as_chars[min_len..]
                .iter()
                .for_each(|char| res.push(*char));
        }

        res
    }
}
