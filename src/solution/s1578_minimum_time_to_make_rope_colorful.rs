/**
 * [1578] Minimum Time to Make Rope Colorful
 *
 * Alice has n balloons arranged on a rope. You are given a 0-indexed string colors where colors[i] is the color of the i^th balloon.
 * Alice wants the rope to be colorful. She does not want two consecutive balloons to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it colorful. You are given a 0-indexed integer array neededTime where neededTime[i] is the time (in seconds) that Bob needs to remove the i^th balloon from the rope.
 * Return the minimum time Bob needs to make the rope colorful.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/13/ballon1.jpg" style="width: 404px; height: 243px;" />
 * Input: colors = "abaac", neededTime = [1,2,3,4,5]
 * Output: 3
 * Explanation: In the above image, 'a' is blue, 'b' is red, and 'c' is green.
 * Bob can remove the blue balloon at index 2. This takes 3 seconds.
 * There are no longer two consecutive balloons of the same color. Total time = 3.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/13/balloon2.jpg" style="width: 244px; height: 243px;" />
 * Input: colors = "abc", neededTime = [1,2,3]
 * Output: 0
 * Explanation: The rope is already colorful. Bob does not need to remove any balloons from the rope.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/13/balloon3.jpg" style="width: 404px; height: 243px;" />
 * Input: colors = "aabaa", neededTime = [1,2,3,4,1]
 * Output: 2
 * Explanation: Bob will remove the balloons at indices 0 and 4. Each balloons takes 1 second to remove.
 * There are no longer two consecutive balloons of the same color. Total time = 1 + 1 = 2.
 *
 *  
 * Constraints:
 *
 * 	n == colors.length == neededTime.length
 * 	1 <= n <= 10^5
 * 	1 <= neededTime[i] <= 10^4
 * 	colors contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
// discuss: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let (mut result, mut l) = (0, 0);

        for r in 1..colors.len() {
            if colors[r..=r] != colors[l..=l] {
                l = r;
            } else if needed_time[r] < needed_time[l] {
                result += needed_time[r];
            } else {
                result += needed_time[l];
                l = r;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1578_example_1() {
        let colors = "abaac".to_string();
        let needed_time = vec![1, 2, 3, 4, 5];

        let result = 3;

        assert_eq!(Solution::min_cost(colors, needed_time), result);
    }

    #[test]
    fn test_1578_example_2() {
        let colors = "abc".to_string();
        let needed_time = vec![1, 2, 3];

        let result = 0;

        assert_eq!(Solution::min_cost(colors, needed_time), result);
    }

    #[test]
    fn test_1578_example_3() {
        let colors = "aabaa".to_string();
        let needed_time = vec![1, 2, 3, 4, 1];

        let result = 2;

        assert_eq!(Solution::min_cost(colors, needed_time), result);
    }
}
