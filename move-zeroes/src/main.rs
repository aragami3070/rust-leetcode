struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut zeroes_count = 0;
        let mut last_not_zero_ind = 0;
        let mut flag = false;
        for ind in 0..nums.len() {
            if nums[ind] != 0 {
                last_not_zero_ind += if flag {
                    1
                } else {
                    flag = true;
                    0
                };
                nums[last_not_zero_ind] = nums[ind]
            } else {
                zeroes_count += 1;
            }
        }
        let len = nums.len();
        for elem in nums.iter_mut().skip(len - zeroes_count) {
            *elem = 0;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0])
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0])
    }

    #[test]
    pub fn success_solution_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0])
    }
}
