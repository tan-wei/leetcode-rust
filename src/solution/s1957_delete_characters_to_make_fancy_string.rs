/**
 * [1957] Delete Characters to Make Fancy String
 *
 * A fancy string is a string where no three consecutive characters are equal.
 * Given a string s, delete the minimum possible number of characters from s to make it fancy.
 * Return the final string after the deletion. It can be shown that the answer will always be unique.
 *  
 * Example 1:
 *
 * Input: s = "le<u>e</u>etcode"
 * Output: "leetcode"
 * Explanation:
 * Remove an 'e' from the first group of 'e's to create "leetcode".
 * No three consecutive characters are equal, so return "leetcode".
 *
 * Example 2:
 *
 * Input: s = "<u>a</u>aab<u>aa</u>aa"
 * Output: "aabaa"
 * Explanation:
 * Remove an 'a' from the first group of 'a's to create "aabaaaa".
 * Remove two 'a's from the second group of 'a's to create "aabaa".
 * No three consecutive characters are equal, so return "aabaa".
 *
 * Example 3:
 *
 * Input: s = "aab"
 * Output: "aab"
 * Explanation: No three consecutive characters are equal, so return "aab".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// discuss: https://leetcode.com/problems/delete-characters-to-make-fancy-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        s.chars()
            .fold(
                ("".to_string(), 0, ' '),
                |(mut result, counter, prev_ch), ch| {
                    if ch == prev_ch {
                        if counter < 1 {
                            result.push_str(ch.to_string().as_str());
                        }
                        (result, counter + 1, ch)
                    } else {
                        result.push_str(ch.to_string().as_str());
                        (result, 0, ch)
                    }
                },
            )
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1957_example_1() {
        let s = "leeetcode".to_string();

        let result = "leetcode".to_string();

        assert_eq!(Solution::make_fancy_string(s), result);
    }

    #[test]
    fn test_1957_example_2() {
        let s = "aaabaaaa".to_string();

        let result = "aabaa".to_string();

        assert_eq!(Solution::make_fancy_string(s), result);
    }

    #[test]
    fn test_1957_example_3() {
        let s = "aab".to_string();

        let result = "aab".to_string();

        assert_eq!(Solution::make_fancy_string(s), result);
    }
}
