/**
 * [0917] Reverse Only Letters
 *
 * Given a string s, reverse the string according to the following rules:
 *
 * 	All the characters that are not English letters remain in the same position.
 * 	All the English letters (lowercase or uppercase) should be reversed.
 *
 * Return s after reversing it.
 *  
 * Example 1:
 * Input: s = "ab-cd"
 * Output: "dc-ba"
 * Example 2:
 * Input: s = "a-bC-dEf-ghIj"
 * Output: "j-Ih-gfE-dCba"
 * Example 3:
 * Input: s = "Test1ng-Leet=code-Q!"
 * Output: "Qedo1ct-eeLg=ntse-T!"
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists of characters with ASCII values in the range [33, 122].
 * 	s does not contain '\"' or '\\'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-only-letters/
// discuss: https://leetcode.com/problems/reverse-only-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut stack = std::collections::VecDeque::new();
        for item in s.chars() {
            if item.is_alphabetic() {
                stack.push_back(item);
            }
        }
        let mut result = String::new();
        for item in s.chars() {
            if item.is_alphabetic() {
                result.push(stack.pop_back().unwrap());
            } else {
                result.push(item);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0917_example_1() {
        let s = "ab-cd".to_string();
        let result = "dc-ba".to_string();

        assert_eq!(Solution::reverse_only_letters(s), result);
    }

    #[test]
    fn test_0917_example_2() {
        let s = "a-bC-dEf-ghIj".to_string();
        let result = "j-Ih-gfE-dCba".to_string();

        assert_eq!(Solution::reverse_only_letters(s), result);
    }

    #[test]
    fn test_0917_example_3() {
        let s = "Test1ng-Leet=code-Q!".to_string();
        let result = "Qedo1ct-eeLg=ntse-T!".to_string();

        assert_eq!(Solution::reverse_only_letters(s), result);
    }
}
