/**
 * [0605] Can Place Flowers
 *
 * You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
 * Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.
 *  
 * Example 1:
 * Input: flowerbed = [1,0,0,0,1], n = 1
 * Output: true
 * Example 2:
 * Input: flowerbed = [1,0,0,0,1], n = 2
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= flowerbed.length <= 2 * 10^4
 * 	flowerbed[i] is 0 or 1.
 * 	There are no two adjacent flowers in flowerbed.
 * 	0 <= n <= flowerbed.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/can-place-flowers/
// discuss: https://leetcode.com/problems/can-place-flowers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/can-place-flowers/discuss/1719845/Rust-iterators-0ms-100-2.1MB-92-chain-fold-tuple-no-mutations-no-declarations
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        n == 0
            || flowerbed
                .iter()
                .chain(std::iter::once(&0))
                .fold((1, 0), |(consecutive_zeros, open_pots), pot| {
                    if *pot == 1 {
                        (0, open_pots)
                    } else if consecutive_zeros == 2 {
                        (1, open_pots + 1)
                    } else {
                        (consecutive_zeros + 1, open_pots)
                    }
                })
                .1
                >= n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0605_example_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let result = true;

        assert_eq!(Solution::can_place_flowers(flowerbed, n), result);
    }

    #[test]
    fn test_0605_example_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let result = false;

        assert_eq!(Solution::can_place_flowers(flowerbed, n), result);
    }
}
