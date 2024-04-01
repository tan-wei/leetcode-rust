/**
 * [1542] Find Longest Awesome Substring
 *
 * You are given a string s. An awesome substring is a non-empty substring of s such that we can make any number of swaps in order to make it a palindrome.
 * Return the length of the maximum length awesome substring of s.
 *  
 * Example 1:
 *
 * Input: s = "3242415"
 * Output: 5
 * Explanation: "24241" is the longest awesome substring, we can form the palindrome "24142" with some swaps.
 *
 * Example 2:
 *
 * Input: s = "12345678"
 * Output: 1
 *
 * Example 3:
 *
 * Input: s = "213123"
 * Output: 6
 * Explanation: "213123" is the longest awesome substring, we can form the palindrome "231132" with some swaps.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists only of digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-longest-awesome-substring/
// discuss: https://leetcode.com/problems/find-longest-awesome-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut mask = 0;
        let mut result = 0;

        map.insert(0, -1);

        for (i, c) in s.bytes().enumerate() {
            mask ^= 1 << (c - b'0');
            if let Some(&j) = map.get(&mask) {
                result = result.max(i as i32 - j);
            }
            for k in 0..10 {
                let mask2 = mask ^ (1 << k);
                if let Some(&j) = map.get(&mask2) {
                    result = result.max(i as i32 - j);
                }
            }
            map.entry(mask).or_insert(i as i32);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1542_example_1() {
        let s = "3242415".to_string();

        let result = 5;

        assert_eq!(Solution::longest_awesome(s), result);
    }

    #[test]
    fn test_1542_example_2() {
        let s = "12345678".to_string();

        let result = 1;

        assert_eq!(Solution::longest_awesome(s), result);
    }

    #[test]
    fn test_1542_example_3() {
        let s = "213123".to_string();

        let result = 6;

        assert_eq!(Solution::longest_awesome(s), result);
    }
}
