/**
 * [2370] Longest Ideal Subsequence
 *
 * You are given a string s consisting of lowercase letters and an integer k. We call a string t ideal if the following conditions are satisfied:
 *
 * 	t is a subsequence of the string s.
 * 	The absolute difference in the alphabet order of every two adjacent letters in t is less than or equal to k.
 *
 * Return the length of the longest ideal string.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 * Note that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of 'a' and 'z' is 25, not 1.
 *  
 * Example 1:
 *
 * Input: s = "acfgbd", k = 2
 * Output: 4
 * Explanation: The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
 * Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.
 * Example 2:
 *
 * Input: s = "abcd", k = 3
 * Output: 4
 * Explanation: The longest ideal string is "abcd". The length of this string is 4, so 4 is returned.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	0 <= k <= 25
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-ideal-subsequence/
// discuss: https://leetcode.com/problems/longest-ideal-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/longest-ideal-subsequence/solutions/5070918/rust-15ms-238mb-one-liner-solution-by-ro-4pj9/
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        s.chars()
            .rev()
            .fold(([0; 26], 1), |(mut v, ans), c| {
                let c = c as usize - 97;
                v[c] = (c.saturating_sub(k as usize)..=25.min(c + k as usize))
                    .fold(1, |mut tmp, j| tmp.max(v[j] + 1));
                (v, ans.max(v[c]))
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2370_example_1() {
        let s = "acfgbd".to_string();
        let k = 2;

        let result = 4;

        assert_eq!(Solution::longest_ideal_string(s, k), result);
    }

    #[test]
    fn test_2370_example_2() {
        let s = "abcd".to_string();
        let k = 3;

        let result = 4;

        assert_eq!(Solution::longest_ideal_string(s, k), result);
    }
}
