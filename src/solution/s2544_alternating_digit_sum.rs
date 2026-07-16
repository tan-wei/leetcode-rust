/**
 * [2544] Alternating Digit Sum
 *
 * You are given a positive integer n. Each digit of n has a sign according to the following rules:
 *
 * 	The most significant digit is assigned a positive sign.
 * 	Each other digit has an opposite sign to its adjacent digits.
 *
 * Return the sum of all digits with their corresponding sign.
 *  
 * Example 1:
 *
 * Input: n = 521
 * Output: 4
 * Explanation: (+5) + (-2) + (+1) = 4.
 *
 * Example 2:
 *
 * Input: n = 111
 * Output: 1
 * Explanation: (+1) + (-1) + (+1) = 1.
 *
 * Example 3:
 *
 * Input: n = 886996
 * Output: 0
 * Explanation: (+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 *  
 * <style type="text/css">.spoilerbutton {display:block; border:dashed; padding: 0px 0px; margin:10px 0px; font-size:150%; font-weight: bold; color:#000000; background-color:cyan; outline:0;
 * }
 * .spoiler {overflow:hidden;}
 * .spoiler > div {-webkit-transition: all 0s ease;-moz-transition: margin 0s ease;-o-transition: all 0s ease;transition: margin 0s ease;}
 * .spoilerbutton[value="Show Message"] + .spoiler > div {margin-top:-500%;}
 * .spoilerbutton[value="Hide Message"] + .spoiler {padding:5px;}
 * </style>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/alternating-digit-sum/
// discuss: https://leetcode.com/problems/alternating-digit-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        n.to_string()
            .bytes()
            .fold((0, 1), |(sum, sig), x| {
                (sum + sig * (x - b'0') as i32, -sig)
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2544_example_1() {
        let n = 521;

        let result = 4;

        assert_eq!(Solution::alternate_digit_sum(n), result);
    }

    #[test]
    fn test_2544_example_2() {
        let n = 111;

        let result = 1;

        assert_eq!(Solution::alternate_digit_sum(n), result);
    }

    #[test]
    fn test_2544_example_3() {
        let n = 886996;

        let result = 0;

        assert_eq!(Solution::alternate_digit_sum(n), result);
    }
}
