/**
 * [1758] Minimum Changes To Make Alternating Binary String
 *
 * You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any '0' to '1' or vice versa.
 * The string is called alternating if no two adjacent characters are equal. For example, the string "010" is alternating, while the string "0100" is not.
 * Return the minimum number of operations needed to make s alternating.
 *  
 * Example 1:
 *
 * Input: s = "0100"
 * Output: 1
 * Explanation: If you change the last character to '1', s will be "0101", which is alternating.
 *
 * Example 2:
 *
 * Input: s = "10"
 * Output: 0
 * Explanation: s is already alternating.
 *
 * Example 3:
 *
 * Input: s = "1111"
 * Output: 2
 * Explanation: You need two operations to reach "0101" or "1010".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// discuss: https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let n = s
            .bytes()
            .enumerate()
            .filter(|&(i, c)| c - b'0' == (i % 2) as u8)
            .count();

        n.min(s.len() - n) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1758_example_1() {
        let s = "0100".to_string();

        let result = 1;

        assert_eq!(Solution::min_operations(s), result);
    }

    #[test]
    fn test_1758_example_2() {
        let s = "10".to_string();

        let result = 0;

        assert_eq!(Solution::min_operations(s), result);
    }

    #[test]
    fn test_1758_example_3() {
        let s = "1111".to_string();

        let result = 2;

        assert_eq!(Solution::min_operations(s), result);
    }
}
