struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = match candies.iter().max() {
            Some(&max_c) => max_c,
            None => panic!("Candies empty"),
        };
        candies
            .iter()
            .map(|kid| kid + extra_candies >= max_candies)
            .collect()
    }
}

fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let res = vec![true, true, true, false, true];
    println!("{}", Solution::kids_with_candies(candies, 3) == res)
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let candies = vec![2, 3, 5, 1, 3];
        let res = vec![true, true, true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, 3), res)
    }

    #[test]
    pub fn success_solution_2() {
        let candies = vec![4, 2, 1, 1, 2];
        let res = vec![true, false, false, false, false];
        assert_eq!(Solution::kids_with_candies(candies, 1), res)
    }

    #[test]
    pub fn success_solution_3() {
        let candies = vec![4, 2, 1, 1, 2];
        let res = vec![true, true, true, true, true];
        assert_eq!(Solution::kids_with_candies(candies, 3), res)
    }

    #[test]
    pub fn success_solution_4() {
        let candies = vec![12, 1, 12];
        let res = vec![true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, 10), res)
    }
}
