struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed
            .iter()
            .fold((0u32, false), |(can_place, prev_planted), &cur| {
                if cur == 0 {
                    if !prev_planted {
                        (can_place + 1, true)
                    } else {
                        (can_place, false)
                    }
                } else {
                    (
                        if !prev_planted {
                            can_place
                        } else {
                            can_place.checked_sub(1).unwrap_or_default()
                        },
                        true,
                    )
                }
            })
            .0 as i32
            >= n
    }
}

fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    println!("{}", Solution::can_place_flowers(flowerbed, 1));
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn succes_solution_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        assert!(Solution::can_place_flowers(flowerbed, 1))
    }

    #[test]
    pub fn succes_solution_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 2))
    }

    #[test]
    pub fn succes_solution_3() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 2))
    }

    #[test]
    pub fn succes_solution_4() {
        let flowerbed = vec![1, 0, 0, 0, 0];
        assert!(Solution::can_place_flowers(flowerbed, 2))
    }

    #[test]
    pub fn succes_solution_5() {
        let flowerbed = vec![0, 0, 0];
        assert!(Solution::can_place_flowers(flowerbed, 2))
    }
}
