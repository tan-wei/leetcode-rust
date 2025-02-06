/**
 * [1876] Substrings of Size Three with Distinct Characters
 *
 * A string is good if there are no repeated characters.
 * Given a string s​​​​​, return the number of good substrings of length three in s​​​​​​.
 * Note that if there are multiple occurrences of the same substring, every occurrence should be counted.
 * A substring is a contiguous sequence of characters in a string.
 *  
 * Example 1:
 *
 * Input: s = "xyzzaz"
 * Output: 1
 * Explanation: There are 4 substrings of size 3: "xyz", "yzz", "zza", and "zaz".
 * The only good substring of length 3 is "xyz".
 *
 * Example 2:
 *
 * Input: s = "aababcabc"
 * Output: 4
 * Explanation: There are 7 substrings of size 3: "aab", "aba", "bab", "abc", "bca", "cab", and "abc".
 * The good substrings are "abc", "bca", "cab", and "abc".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s​​​​​​ consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/
// discuss: https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes()
            .windows(3)
            .filter(|sub| sub.iter().collect::<std::collections::HashSet<&u8>>().len() == 3)
            .count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1876_example_1() {
        let s = "xyzzaz".to_string();

        let result = 1;

        assert_eq!(Solution::count_good_substrings(s), result);
    }

    #[test]
    fn test_1876_example_2() {
        let s = "aababcabc".to_string();

        let result = 4;

        assert_eq!(Solution::count_good_substrings(s), result);
    }
}
