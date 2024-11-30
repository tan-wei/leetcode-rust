/**
 * [1784] Check if Binary String Has at Most One Segment of Ones
 *
 * Given a binary string s ​​​​​without leading zeros, return true​​​ if s contains at most one contiguous segment of ones. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: s = "1001"
 * Output: false
 * Explanation: The ones do not form a contiguous segment.
 *
 * Example 2:
 *
 * Input: s = "110"
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s[i]​​​​ is either '0' or '1'.
 * 	s[0] is '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
// discuss: https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.chars()
            .skip_while(|val| *val == '1')
            .find(|val| *val == '1')
            .is_none()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1784_example_1() {
        let s = "1001".to_string();

        let result = false;

        assert_eq!(Solution::check_ones_segment(s), result);
    }

    #[test]
    fn test_1784_example_2() {
        let s = "110".to_string();

        let result = true;

        assert_eq!(Solution::check_ones_segment(s), result);
    }
}
