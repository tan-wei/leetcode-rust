/**
 * [1344] Angle Between Hands of a Clock
 *
 * Given two numbers, hour and minutes, return the smaller angle (in degrees) formed between the hour and the minute hand.
 * Answers within 10^-5 of the actual value will be accepted as correct.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_1_1673.png" style="width: 300px; height: 296px;" />
 * Input: hour = 12, minutes = 30
 * Output: 165
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_2_1673.png" style="width: 300px; height: 301px;" />
 * Input: hour = 3, minutes = 30
 * Output: 75
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_3_1673.png" style="width: 300px; height: 301px;" />
 * Input: hour = 3, minutes = 15
 * Output: 7.5
 *
 *  
 * Constraints:
 *
 * 	1 <= hour <= 12
 * 	0 <= minutes <= 59
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/angle-between-hands-of-a-clock/
// discuss: https://leetcode.com/problems/angle-between-hands-of-a-clock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mang = 6. * minutes as f64;
        let hang = 30. * (hour % 12) as f64 + mang / 12.;

        let diff = hang - mang;

        diff.abs().min(360. - diff.abs())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1344_example_1() {
        let hour = 12;
        let minutes = 30;

        let result = 165.0;

        assert_f64_near!(Solution::angle_clock(hour, minutes), result);
    }

    #[test]
    #[ignore]
    fn test_1344_example_2() {
        let hour = 3;
        let minutes = 30;

        let result = 75.0;

        assert_f64_near!(Solution::angle_clock(hour, minutes), result);
    }

    #[test]
    #[ignore]
    fn test_1344_example_3() {
        let hour = 3;
        let minutes = 10;

        let result = 7.5;

        assert_f64_near!(Solution::angle_clock(hour, minutes), result);
    }
}
