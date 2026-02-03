fn is_vowels(symbol: u8) -> bool {
    symbol == b'a'
        || symbol == b'A'
        || symbol == b'e'
        || symbol == b'E'
        || symbol == b'i'
        || symbol == b'I'
        || symbol == b'o'
        || symbol == b'O'
        || symbol == b'u'
        || symbol == b'U'
}
struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s;
        let vowels = s
            .as_bytes()
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, symbol)| is_vowels(symbol))
            .collect::<Vec<(usize, u8)>>();

        for (ind, &(_, symb)) in vowels.iter().enumerate() {
            let (str_ind, _) = vowels[vowels.len() - 1 - ind];
            s.replace_range(
                str_ind..str_ind + 1,
                &String::from_utf8(vec![symb]).unwrap(),
            );
        }

        s
    }
}

fn main() {
    println!("{}", Solution::reverse_vowels("IceCreAm".to_string()));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let word = String::from("IceCreAm");
        assert_eq!(Solution::reverse_vowels(word), String::from("AceCreIm"));
    }

    #[test]
    pub fn success_solution_2() {
        let word = String::from("leetcode");
        assert_eq!(Solution::reverse_vowels(word), String::from("leotcede"));
    }

    #[test]
    pub fn success_solution_3() {
        let word = String::from("KKK");
        assert_eq!(Solution::reverse_vowels(word), String::from("KKK"));
    }

    #[test]
    pub fn success_solution_4() {
        let word = String::from("AEI");
        assert_eq!(Solution::reverse_vowels(word), String::from("IEA"));
    }
}
