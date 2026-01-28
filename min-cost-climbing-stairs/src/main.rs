struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut din_p_min_cost_calc: Vec<i32> = vec![0; cost.len() + 2];
        for ind in (0..cost.len()).rev() {
            din_p_min_cost_calc[ind] = cost[ind]
                + std::cmp::min(din_p_min_cost_calc[ind + 1], din_p_min_cost_calc[ind + 2]);
        }

        std::cmp::min(din_p_min_cost_calc[0], din_p_min_cost_calc[1])
    }
}

fn main() {
    let cost = vec![100, 100, 100, 100, 1, 1, 100];
    Solution::min_cost_climbing_stairs(cost);
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let cost = vec![10, 15, 20];
        let res = 15;
        assert_eq!(Solution::min_cost_climbing_stairs(cost), res)
    }

    #[test]
    pub fn success_solution_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let res = 6;
        assert_eq!(Solution::min_cost_climbing_stairs(cost), res)
    }

    #[test]
    pub fn success_solution_3() {
        let cost = vec![100, 100, 100, 100, 1, 1, 100];
        let res = 201;
        assert_eq!(Solution::min_cost_climbing_stairs(cost), res)
    }

    #[test]
    pub fn success_solution_4() {
        let cost = vec![1, 1, 1, 1, 100, 100, 1];
        let res = 102;
        assert_eq!(Solution::min_cost_climbing_stairs(cost), res)
    }
}
