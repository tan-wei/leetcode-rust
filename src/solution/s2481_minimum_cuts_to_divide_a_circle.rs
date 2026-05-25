/**
 * [2481] Minimum Cuts to Divide a Circle
 *
 * A valid cut in a circle can be:
 *
 * 	A cut that is represented by a straight line that touches two points on the edge of the circle and passes through its center, or
 * 	A cut that is represented by a straight line that touches one point on the edge of the circle and its center.
 *
 * Some valid and invalid cuts are shown in the figures below.
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/29/alldrawio.png" style="width: 450px; height: 174px;" />
 * Given the integer n, return the minimum number of cuts needed to divide a circle into n equal slices.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/24/11drawio.png" style="width: 200px; height: 200px;" />
 * Input: n = 4
 * Output: 2
 * Explanation:
 * The above figure shows how cutting the circle twice through the middle divides it into 4 equal slices.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/24/22drawio.png" style="width: 200px; height: 201px;" />
 * Input: n = 3
 * Output: 3
 * Explanation:
 * At least 3 cuts are needed to divide the circle into 3 equal slices.
 * It can be shown that less than 3 cuts cannot result in 3 slices of equal size and shape.
 * Also note that the first cut will not divide the circle into distinct parts.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/
// discuss: https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        match n % 2 {
            1 if n == 1 => 0,
            1 => n,
            _ => n / 2,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2481_example_1() {
        let n = 4;

        let result = 2;

        assert_eq!(Solution::number_of_cuts(n), result);
    }

    #[test]
    fn test_2481_example_2() {
        let n = 3;

        let result = 3;

        assert_eq!(Solution::number_of_cuts(n), result);
    }
}
