use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, Vec<i32>> = HashMap::new();

        // NOTE: Fill map with pairs (nums_value, [nums_indexes])
        for (index, &num) in nums.iter().enumerate() {
            // O(n)
            let i_x: i32 = index.try_into().unwrap();
            nums_map
                .entry(num)
                .and_modify(|indexes| {
                    indexes.push(i_x);
                })
                .or_insert(vec![i_x]);
        }

        for (index, &num) in nums.iter().enumerate() {
            // O(n)
            let diff = target - num;

            if let Some(second) = nums_map.get(&diff) { // O(log(n))
                let ind: i32 = index.try_into().unwrap();
                for &i in second.iter() {
                    if ind != i {
                        return vec![ind, i];
                    }
                }
            }
        }

        panic!("Not found")
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15, 7], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
}
