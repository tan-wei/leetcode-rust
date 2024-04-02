/**
 * [1544] Make The String Great
 *
 * Given a string s of lower and upper case English letters.
 * A good string is a string which doesn't have two adjacent characters s[i] and s[i + 1] where:
 *
 * 	0 <= i <= s.length - 2
 * 	s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-case or vice-versa.
 *
 * To make the string good, you can choose two adjacent characters that make the string bad and remove them. You can keep doing this until the string becomes good.
 * Return the string after making it good. The answer is guaranteed to be unique under the given constraints.
 * Notice that an empty string is also good.
 *  
 * Example 1:
 *
 * Input: s = "leEeetcode"
 * Output: "leetcode"
 * Explanation: In the first step, either you choose i = 1 or i = 2, both will result "leEeetcode" to be reduced to "leetcode".
 *
 * Example 2:
 *
 * Input: s = "abBAcC"
 * Output: ""
 * Explanation: We have many possible scenarios, and all lead to the same answer. For example:
 * "abBAcC" --> "aAcC" --> "cC" --> ""
 * "abBAcC" --> "abBA" --> "aA" --> ""
 *
 * Example 3:
 *
 * Input: s = "s"
 * Output: "s"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s contains only lower and upper case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-the-string-great/
// discuss: https://leetcode.com/problems/make-the-string-great/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_good(s: String) -> String {
        s.chars()
            .fold(String::new(), |mut result, c2| match (result.pop(), c2) {
                (None, c2) => c2.to_string(),
                (Some(c1), c2) if c1 != c2 && c1.to_lowercase().eq(c2.to_lowercase()) => result,
                (Some(c1), c2) => result + &c1.to_string() + &c2.to_string(),
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1544_example_1() {
        let s = "leEeetcode".to_string();

        let result = "leetcode".to_string();

        assert_eq!(Solution::make_good(s), result);
    }

    #[test]
    fn test_1544_example_2() {
        let s = "abBAcC".to_string();

        let result = "".to_string();

        assert_eq!(Solution::make_good(s), result);
    }

    #[test]
    fn test_1544_example_3() {
        let s = "s".to_string();

        let result = "s".to_string();

        assert_eq!(Solution::make_good(s), result);
    }
}
