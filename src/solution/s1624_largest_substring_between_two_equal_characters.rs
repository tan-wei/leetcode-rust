/**
 * [1624] Largest Substring Between Two Equal Characters
 *
 * Given a string s, return the length of the longest substring between two equal characters, excluding the two characters. If there is no such substring return -1.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "aa"
 * Output: 0
 * Explanation: The optimal substring here is an empty substring between the two 'a's.
 * Example 2:
 *
 * Input: s = "abca"
 * Output: 2
 * Explanation: The optimal substring here is "bc".
 *
 * Example 3:
 *
 * Input: s = "cbzxy"
 * Output: -1
 * Explanation: There are no characters that appear twice in s.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 300
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-substring-between-two-equal-characters/
// discuss: https://leetcode.com/problems/largest-substring-between-two-equal-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        s.as_bytes()
            .into_iter()
            .enumerate()
            .fold(vec![vec![]; 26], |mut v, (ind, c)| {
                v[(c - b'a') as usize].push(ind as i32);
                v
            })
            .into_iter()
            .filter(|indexes| !indexes.is_empty())
            .map(|indexes| indexes.last().unwrap() - indexes.first().unwrap() - 1)
            .max()
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1624_example_1() {
        let s = "aa".to_string();

        let result = 0;

        assert_eq!(Solution::max_length_between_equal_characters(s), result);
    }

    #[test]
    fn test_1624_example_2() {
        let s = "abca".to_string();

        let result = 2;

        assert_eq!(Solution::max_length_between_equal_characters(s), result);
    }

    #[test]
    fn test_1624_example_3() {
        let s = "cbzxy".to_string();

        let result = -1;

        assert_eq!(Solution::max_length_between_equal_characters(s), result);
    }
}
