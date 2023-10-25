/**
 * [1358] Number of Substrings Containing All Three Characters
 *
 * Given a string s consisting only of characters a, b and c.
 * Return the number of substrings containing at least one occurrence of all these characters a, b and c.
 *  
 * Example 1:
 *
 * Input: s = "abcabc"
 * Output: 10
 * Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
 *
 * Example 2:
 *
 * Input: s = "aaacb"
 * Output: 3
 * Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
 *
 * Example 3:
 *
 * Input: s = "abc"
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 5 x 10^4
 * 	s only consists of a, b or c characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
// discuss: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut count = vec![0; 3];
        let (mut left, mut right) = (0, 0);
        let s = s.as_bytes();
        while right < s.len() {
            count[(s[right] - b'a') as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                count[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            result += left;
            right += 1;
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1358_example_1() {
        let s = "abcabc".to_string();

        let result = 10;

        assert_eq!(Solution::number_of_substrings(s), result);
    }

    #[test]
    fn test_1358_example_2() {
        let s = "aaacb".to_string();

        let result = 3;

        assert_eq!(Solution::number_of_substrings(s), result);
    }

    #[test]
    fn test_1358_example_3() {
        let s = "abc".to_string();

        let result = 1;

        assert_eq!(Solution::number_of_substrings(s), result);
    }
}
