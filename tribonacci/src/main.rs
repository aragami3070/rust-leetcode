struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 3 {
            if n == 0 {
                return 0;
            }
            return 1;
        }
        let (mut first, mut second, mut third) = (0, 1, 1);

        for _ in 3..n + 1 {
            let temp = first + second + third;
            first = second;
            second = third;
            third = temp;
        }

        third
    }
}

fn main() {
    println!("{}", Solution::tribonacci(7) == 24)
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        assert_eq!(Solution::tribonacci(4), 4)
    }

    #[test]
    pub fn success_solution_2() {
        assert_eq!(Solution::tribonacci(5), 7)
    }

    #[test]
    pub fn success_solution_3() {
        assert_eq!(Solution::tribonacci(6), 13)
    }

    #[test]
    pub fn success_solution_4() {
        assert_eq!(Solution::tribonacci(7), 24)
    }

    #[test]
    pub fn success_solution_5() {
        assert_eq!(Solution::tribonacci(0), 0)
    }

    #[test]
    pub fn success_solution_6() {
        assert_eq!(Solution::tribonacci(1), 1)
    }

    #[test]
    pub fn success_solution_7() {
        assert_eq!(Solution::tribonacci(2), 1)
    }
}
