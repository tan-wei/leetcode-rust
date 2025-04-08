/**
 * [1954] Minimum Garden Perimeter to Collect Enough Apples
 *
 * In a garden represented as an infinite 2D grid, there is an apple tree planted at every integer coordinate. The apple tree planted at an integer coordinate (i, j) has |i| + |j| apples growing on it.
 * You will buy an axis-aligned square plot of land that is centered at (0, 0).
 * Given an integer neededApples, return the minimum perimeter of a plot such that at least neededApples apples are inside or on the perimeter of that plot.
 * The value of |x| is defined as:
 *
 * 	x if x >= 0
 * 	-x if x < 0
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/30/1527_example_1_2.png" style="width: 442px; height: 449px;" />
 * Input: neededApples = 1
 * Output: 8
 * Explanation: A square plot of side length 1 does not contain any apples.
 * However, a square plot of side length 2 has 12 apples inside (as depicted in the image above).
 * The perimeter is 2 * 4 = 8.
 *
 * Example 2:
 *
 * Input: neededApples = 13
 * Output: 16
 *
 * Example 3:
 *
 * Input: neededApples = 1000000000
 * Output: 5040
 *
 *  
 * Constraints:
 *
 * 	1 <= neededApples <= 10^15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-garden-perimeter-to-collect-enough-apples/
// discuss: https://leetcode.com/problems/minimum-garden-perimeter-to-collect-enough-apples/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut cubrt = 3.0f64.powf((needed_apples as f64 / 4.0).log(3.0) / 3.0) as i64;
        while needed_apples <= (cubrt * (cubrt + 1) * ((cubrt << 1) + 1)) << 1 {
            cubrt -= 1;
        }
        8 * (cubrt + 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1954_example_1() {
        let needed_apples = 1;

        let result = 8;

        assert_eq!(Solution::minimum_perimeter(needed_apples), result);
    }

    #[test]
    fn test_1954_example_2() {
        let needed_apples = 13;

        let result = 16;

        assert_eq!(Solution::minimum_perimeter(needed_apples), result);
    }

    #[test]
    fn test_1954_example_3() {
        let needed_apples = 1000000000;

        let result = 5040;

        assert_eq!(Solution::minimum_perimeter(needed_apples), result);
    }
}
