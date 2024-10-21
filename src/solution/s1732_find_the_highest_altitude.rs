/**
 * [1732] Find the Highest Altitude
 *
 * There is a biker going on a road trip. The road trip consists of n + 1 points at different altitudes. The biker starts his trip on point 0 with altitude equal 0.
 * You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i​​​​​​ and i + 1 for all (0 <= i < n). Return the highest altitude of a point.
 *  
 * Example 1:
 *
 * Input: gain = [-5,1,5,0,-7]
 * Output: 1
 * Explanation: The altitudes are [0,-5,-4,1,1,-6]. The highest is 1.
 *
 * Example 2:
 *
 * Input: gain = [-4,-3,-2,-1,4,3,2]
 * Output: 0
 * Explanation: The altitudes are [0,-4,-7,-9,-10,-6,-3,-1]. The highest is 0.
 *
 *  
 * Constraints:
 *
 * 	n == gain.length
 * 	1 <= n <= 100
 * 	-100 <= gain[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-highest-altitude/
// discuss: https://leetcode.com/problems/find-the-highest-altitude/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(height, max), g| (height + g, max.max(height + g)))
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1732_example_1() {
        let gain = vec![-5, 1, 5, 0, -7];

        let result = 1;

        assert_eq!(Solution::largest_altitude(gain), result);
    }

    #[test]
    fn test_1732_example_2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];

        let result = 0;

        assert_eq!(Solution::largest_altitude(gain), result);
    }
}
