struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() == str2.len() && str1 == str2 {
            return str1;
        }
        let max_str: Vec<u8>;
        let mut res_str: Vec<u8>;
        let min_str = if str1.len() >= str2.len() {
            max_str = str1.as_bytes().to_vec();
            res_str = str2.as_bytes().to_vec();
            res_str.clone()
        } else {
            max_str = str2.as_bytes().to_vec();
            res_str = str1.as_bytes().to_vec();
            res_str.clone()
        };

        'main_loop: while !res_str.is_empty() {
            while !max_str.len().is_multiple_of(res_str.len())
                || !min_str.len().is_multiple_of(res_str.len())
            {
                res_str.pop();
                if res_str.is_empty() {
                    return "".to_string();
                }
            }
            for (index, &symbol) in max_str.iter().enumerate() {
                let res_index = index % res_str.len();
                if symbol != res_str[res_index]
                    || min_str[index % min_str.len()] != res_str[res_index]
                {
                    res_str.pop();
                    continue 'main_loop;
                }
            }
            return match String::from_utf8(res_str) {
                Ok(s) => s,
                Err(_) => "".to_string(),
            };
        }

        "".to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::gcd_of_strings(
            "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
            "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string()
        )
    )
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn correct_gcd_of_strings_1() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn correct_gcd_of_strings_2() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABCABC".to_string()),
            "ABCABC"
        );
    }

    #[test]
    fn correct_gcd_of_strings_3() {
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "AB".to_string()),
            "AB"
        );
    }

    #[test]
    fn correct_gcd_of_strings_4() {
        assert_eq!(
            Solution::gcd_of_strings("AAAAAAA".to_string(), "AA".to_string()),
            "A"
        );
    }

    #[test]
    fn correct_gcd_of_strings_5() {
        assert_eq!(
            Solution::gcd_of_strings("ABC".to_string(), "ABCABC".to_string()),
            "ABC"
        );
    }

    #[test]
    fn correct_gcd_of_strings_6() {
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB"
        );
    }

    #[test]
    fn correct_gcd_of_strings_7() {
        assert_eq!(
            Solution::gcd_of_strings(
                "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
                "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string()
            ),
            "TAUXX"
        );
    }

    #[test]
    fn correct_gcd_of_strings_8() {
        assert_eq!(
            Solution::gcd_of_strings(
                "AAAAAAAAA".to_string(),
                "AAACCC".to_string()
            ),
            ""
        );
    }
}
