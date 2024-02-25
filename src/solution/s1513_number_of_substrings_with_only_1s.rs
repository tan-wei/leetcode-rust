/**
 * [1513] Number of Substrings With Only 1s
 *
 * Given a binary string s, return the number of substrings with all characters 1's. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: s = "0110111"
 * Output: 9
 * Explanation: There are 9 substring in total with only 1's characters.
 * "1" -> 5 times.
 * "11" -> 3 times.
 * "111" -> 1 time.
 * Example 2:
 *
 * Input: s = "101"
 * Output: 2
 * Explanation: Substring "1" is shown 2 times in s.
 *
 * Example 3:
 *
 * Input: s = "111111"
 * Output: 21
 * Explanation: Each substring contains only 1's characters.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-substrings-with-only-1s/
// discuss: https://leetcode.com/problems/number-of-substrings-with-only-1s/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut curr_cnt: i64 = 0;
        let mut result: i64 = 0;

        for c in s.chars() {
            if c == '1' {
                curr_cnt += 1;
            } else {
                result += (curr_cnt * (curr_cnt + 1) / 2) % 10_000_000_007;
                curr_cnt = 0;
            }
        }
        result += (curr_cnt * (curr_cnt + 1) / 2) % 10_000_000_007;

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1513_example_1() {
        let s = "0110111".to_string();

        let result = 9;

        assert_eq!(Solution::num_sub(s), result);
    }

    #[test]
    fn test_1513_example_2() {
        let s = "101".to_string();

        let result = 2;

        assert_eq!(Solution::num_sub(s), result);
    }

    #[test]
    fn test_1513_example_3() {
        let s = "111111".to_string();

        let result = 21;

        assert_eq!(Solution::num_sub(s), result);
    }
}
