struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut money_sums = vec![0; nums.len() + 1];
        money_sums[1] = nums[0];

        for (ind, money) in nums.iter().enumerate().skip(1) {
            money_sums[ind + 1] = std::cmp::max(money_sums[ind], money + money_sums[ind - 1])
        }

        money_sums[nums.len()]
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    println!("{}", Solution::rob(nums) == 4)
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4)
    }

    #[test]
    pub fn success_solution_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12)
    }

    #[test]
    pub fn success_solution_3() {
        let nums = vec![1, 1, 100, 150, 100, 1];
        assert_eq!(Solution::rob(nums), 201)
    }

    #[test]
    pub fn success_solution_4() {
        let nums = vec![20, 10, 19, 20];
        assert_eq!(Solution::rob(nums), 40)
    }

    #[test]
    pub fn success_solution_5() {
        let nums = vec![2, 8, 5];
        assert_eq!(Solution::rob(nums), 8)
    }

    #[test]
    pub fn success_solution_6() {
        let nums = vec![10];
        assert_eq!(Solution::rob(nums), 10)
    }
}
