/**
 * [1016] Binary String With Substrings Representing 1 To N
 *
 * Given a binary string s and a positive integer n, return true if the binary representation of all the integers in the range [1, n] are substrings of s, or false otherwise.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 * Input: s = "0110", n = 3
 * Output: true
 * Example 2:
 * Input: s = "0110", n = 4
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s[i] is either '0' or '1'.
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/
// discuss: https://leetcode.com/problems/binary-string-with-substrings-representing-1-to-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for num in (1..=n).rev() {
            let bin_string = format!("{:b}", num);
            if !s.contains(&bin_string) {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1016_example_1() {
        let s = "0110".to_string();
        let n = 3;
        let result = true;

        assert_eq!(Solution::query_string(s, n), result);
    }

    #[test]
    fn test_1016_example_2() {
        let s = "0110".to_string();
        let n = 4;
        let result = false;

        assert_eq!(Solution::query_string(s, n), result);
    }
}
