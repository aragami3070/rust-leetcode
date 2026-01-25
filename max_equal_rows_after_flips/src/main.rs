use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut equal_row_conuter = HashMap::<Vec<u8>, i32>::new();
        for row in matrix.iter() {
            // create bin num by algorithm
            // if elem row == first elem row then push 1
            // else 0
            let first_bit = row[0];
            let binary_num = row
                .iter()
                .map(|&bit| if bit == first_bit { 1 } else { 0 })
                .collect();

            // inc counter with this binary
            *equal_row_conuter.entry(binary_num).or_insert(0) += 1;
        }
        match equal_row_conuter.iter().max_by_key(|&(_, value)| value) {
            Some((_, &res)) => res,
            None => panic!("Why map empty? :/"),
        }
    }
}

fn main() {
    let matrix = vec![vec![0, 1], vec![1, 1]];
    println!("{} = {}", Solution::max_equal_rows_after_flips(matrix), 1);
    let matrix = vec![vec![0, 1], vec![1, 0]];
    println!("{} = {}", Solution::max_equal_rows_after_flips(matrix), 2);
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn success_solution_1() {
        let matrix = vec![vec![0, 1], vec![1, 1]];
        assert_eq!(Solution::max_equal_rows_after_flips(matrix), 1);
    }

    #[test]
    pub fn success_solution_2() {
        let matrix = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::max_equal_rows_after_flips(matrix), 2);
    }

    #[test]
    pub fn success_solution_3() {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
        assert_eq!(Solution::max_equal_rows_after_flips(matrix), 2);
    }

    #[test]
    pub fn success_solution_4() {
        let matrix = vec![
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 1],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
        ];
        assert_eq!(Solution::max_equal_rows_after_flips(matrix), 3);
    }

    #[test]
    pub fn success_solution_5() {
        let matrix = vec![vec![1; 300]; 300];
        assert_eq!(Solution::max_equal_rows_after_flips(matrix), 300);
    }
}
