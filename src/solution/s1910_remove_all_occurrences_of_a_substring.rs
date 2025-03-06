/**
 * [1910] Remove All Occurrences of a Substring
 *
 * Given two strings s and part, perform the following operation on s until all occurrences of the substring part are removed:
 *
 * 	Find the leftmost occurrence of the substring part and remove it from s.
 *
 * Return s after removing all occurrences of part.
 * A substring is a contiguous sequence of characters in a string.
 *  
 * Example 1:
 *
 * Input: s = "daabcbaabcbc", part = "abc"
 * Output: "dab"
 * Explanation: The following operations are done:
 * - s = "da<u>abc</u>baabcbc", remove "abc" starting at index 2, so s = "dabaabcbc".
 * - s = "daba<u>abc</u>bc", remove "abc" starting at index 4, so s = "dababc".
 * - s = "dab<u>abc</u>", remove "abc" starting at index 3, so s = "dab".
 * Now s has no occurrences of "abc".
 *
 * Example 2:
 *
 * Input: s = "axxxxyyyyb", part = "xy"
 * Output: "ab"
 * Explanation: The following operations are done:
 * - s = "axxx<u>xy</u>yyyb", remove "xy" starting at index 4 so s = "axxxyyyb".
 * - s = "axx<u>xy</u>yyb", remove "xy" starting at index 3 so s = "axxyyb".
 * - s = "ax<u>xy</u>yb", remove "xy" starting at index 2 so s = "axyb".
 * - s = "a<u>xy</u>b", remove "xy" starting at index 1 so s = "ab".
 * Now s has no occurrences of "xy".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	1 <= part.length <= 1000
 * 	s​​​​​​ and part consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
// discuss: https://leetcode.com/problems/remove-all-occurrences-of-a-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        s.chars()
            .fold(vec![], |mut stack, c| {
                stack.push(c);
                if stack.len() < part.len() {
                    return stack;
                }
                if part
                    .chars()
                    .zip(stack.iter().skip(stack.len() - part.len()))
                    .all(|(i1, i2)| i1 == *i2)
                {
                    stack = stack
                        .iter()
                        .take(stack.len() - part.len())
                        .map(|c| *c)
                        .collect::<Vec<_>>();
                }
                stack
            })
            .into_iter()
            .collect::<_>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1910_example_1() {
        let s = "daabcbaabcbc".to_string();
        let part = "abc".to_string();

        let result = "dab".to_string();

        assert_eq!(Solution::remove_occurrences(s, part), result);
    }

    #[test]
    fn test_1910_example_2() {
        let s = "axxxxyyyyb".to_string();
        let part = "xy".to_string();

        let result = "ab".to_string();

        assert_eq!(Solution::remove_occurrences(s, part), result);
    }
}
