struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let len = (x as f64).log10().floor() as usize + 1;

        let str_x: Vec<u8> = x.to_string().as_bytes().to_vec();
        for i in 0..len / 2 {
            if str_x[i] != str_x[len - 1 - i] {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(1001));
    println!("{}", Solution::is_palindrome(2));
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(11));
    println!("{}", Solution::is_palindrome(123123));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(212901231));
}
