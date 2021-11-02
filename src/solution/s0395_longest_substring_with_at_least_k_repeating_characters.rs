/**
 * [0395] Longest Substring with At Least K Repeating Characters
 *
 * Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k.
 *  
 * Example 1:
 *
 * Input: s = "aaabb", k = 3
 * Output: 3
 * Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
 *
 * Example 2:
 *
 * Input: s = "ababbc", k = 2
 * Output: 5
 * Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s consists of only lowercase English letters.
 * 	1 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        for i in 1..=26 {
            let mut dict = [0; 26];
            let (mut l, mut r) = (0, 0);
            let mut uniq = 0;
            let mut ok = 0;
            while r < s.len() {
                if uniq <= i {
                    let idx = (s[r] - b'a') as usize;
                    if dict[idx] == 0 {
                        uniq += 1;
                    }
                    dict[idx] += 1;
                    if dict[idx] == k {
                        ok += 1;
                    }
                    r += 1;
                } else {
                    let idx = (s[l] - b'a') as usize;
                    if dict[idx] == k {
                        ok -= 1;
                    }
                    dict[idx] -= 1;
                    if dict[idx] == 0 {
                        uniq -= 1;
                    }
                    l += 1;
                }
                if uniq == i && uniq == ok {
                    result = std::cmp::max(result, r - l)
                }
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0395_example_1() {
        let s = "aaabb".to_string();
        let k = 3;
        let result = 3;

        assert_eq!(Solution::longest_substring(s, k), result);
    }

    #[test]
    fn test_0395_example_2() {
        let s = "ababbc".to_string();
        let k = 2;
        let result = 5;

        assert_eq!(Solution::longest_substring(s, k), result);
    }
}
