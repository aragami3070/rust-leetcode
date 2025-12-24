pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut word1_as_chars = word1.chars().peekable();
        let mut word2_as_chars = word2.chars().peekable();
        let mut res = String::new();

        while word1_as_chars.peek().is_some() || word2_as_chars.peek().is_some() {
            if let Some(ch) = word1_as_chars.next() {
                res.push(ch);
            }

            if let Some(ch) = word2_as_chars.next() {
                res.push(ch);
            }
        }

        res
    }
}
