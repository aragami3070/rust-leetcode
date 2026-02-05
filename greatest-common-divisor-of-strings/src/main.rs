struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() == str2.len() && str1 == str2 {
            return str1;
        }

        let len_str1 = str1.len();
        let len_str2 = str2.len();
        if str1.clone() + &str2 != str2 + &str1 {
            return String::from("");
        }
        let last_symbol_idx = Self::gcd(len_str1, len_str2);

        str1[..last_symbol_idx].to_string()
    }

    fn gcd(len1: usize, len2: usize) -> usize {
        if len2 == 0 {
            len1
        } else {
            Self::gcd(len2, len1 % len2)
        }
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
            Solution::gcd_of_strings("AAAAAAAAA".to_string(), "AAACCC".to_string()),
            ""
        );
    }
}
