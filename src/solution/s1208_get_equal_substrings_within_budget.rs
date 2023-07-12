/**
 * [1208] Get Equal Substrings Within Budget
 *
 * You are given two strings s and t of the same length and an integer maxCost.
 * You want to change s to t. Changing the i^th character of s to i^th character of t costs |s[i] - t[i]| (i.e., the absolute difference between the ASCII values of the characters).
 * Return the maximum length of a substring of s that can be changed to be the same as the corresponding substring of t with a cost less than or equal to maxCost. If there is no substring from s that can be changed to its corresponding substring from t, return 0.
 *  
 * Example 1:
 *
 * Input: s = "abcd", t = "bcdf", maxCost = 3
 * Output: 3
 * Explanation: "abc" of s can change to "bcd".
 * That costs 3, so the maximum length is 3.
 *
 * Example 2:
 *
 * Input: s = "abcd", t = "cdef", maxCost = 3
 * Output: 1
 * Explanation: Each character in s costs 2 to change to character in t,  so the maximum length is 1.
 *
 * Example 3:
 *
 * Input: s = "abcd", t = "acde", maxCost = 0
 * Output: 1
 * Explanation: You cannot make any change, so the maximum length is 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	t.length == s.length
 * 	0 <= maxCost <= 10^6
 * 	s and t consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/get-equal-substrings-within-budget/
// discuss: https://leetcode.com/problems/get-equal-substrings-within-budget/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();
        let mut sum_cost = 0;
        let mut max_len = 0;
        let mut curr_len = 0;
        let mut left = 0;

        for right in 0..sv.len() {
            sum_cost += (sv[right] as i32 - tv[right] as i32).abs();
            curr_len += 1;

            while sum_cost > max_cost {
                sum_cost -= (sv[left] as i32 - tv[left] as i32).abs();
                curr_len -= 1;
                left += 1;
            }

            max_len = max_len.max(curr_len);
        }

        max_len
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1208_example_1() {
        let s = "abcd".to_string();
        let t = "bcdf".to_string();
        let max_cost = 3;
        let result = 3;

        assert_eq!(Solution::equal_substring(s, t, max_cost), result);
    }

    #[test]
    fn test_1208_example_2() {
        let s = "abcd".to_string();
        let t = "cdef".to_string();
        let max_cost = 3;
        let result = 1;

        assert_eq!(Solution::equal_substring(s, t, max_cost), result);
    }

    #[test]
    fn test_1208_example_3() {
        let s = "abcd".to_string();
        let t = "acde".to_string();
        let max_cost = 0;
        let result = 1;

        assert_eq!(Solution::equal_substring(s, t, max_cost), result);
    }
}
