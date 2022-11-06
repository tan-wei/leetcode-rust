/**
 * [0875] Koko Eating Bananas
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has piles[i] bananas. The guards have gone and will come back in h hours.
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
 * Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
 * Return the minimum integer k such that she can eat all the bananas within h hours.
 *  
 * Example 1:
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 * Example 2:
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 * Example 3:
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *  
 * Constraints:
 *
 * 	1 <= piles.length <= 10^4
 * 	piles.length <= h <= 10^9
 * 	1 <= piles[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/koko-eating-bananas/
// discuss: https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = 1000_000_000;

        while left < right {
            let mid: i32 = left + (right - left) / 2;
            let mut count: i32 = 0;

            for pile in piles.iter() {
                count += (*pile as f64 / mid as f64).ceil() as i32;
                if count > h {
                    break;
                }
            }
            if count > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0875_example_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let result = 4;

        assert_eq!(Solution::min_eating_speed(piles, h), result);
    }

    #[test]
    fn test_0875_example_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let result = 30;

        assert_eq!(Solution::min_eating_speed(piles, h), result);
    }

    #[test]
    fn test_0875_example_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        let result = 23;

        assert_eq!(Solution::min_eating_speed(piles, h), result);
    }
}
