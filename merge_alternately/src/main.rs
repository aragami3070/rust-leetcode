use merge_alternately::Solution;

fn main() {
    println!("{}", Solution::merge_alternately("ab".to_string(), "pqrrr".to_string()));
    println!("{}", Solution::merge_alternately("abccc".to_string(), "pq".to_string()));
}
