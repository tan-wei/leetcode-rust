/**
 * [2390] Removing Stars From a String
 *
 * You are given a string s, which contains stars *.
 * In one operation, you can:
 *
 * 	Choose a star in s.
 * 	Remove the closest non-star character to its left, as well as remove the star itself.
 *
 * Return the string after all stars have been removed.
 * Note:
 *
 * 	The input will be generated such that the operation is always possible.
 * 	It can be shown that the resulting string will always be unique.
 *
 *  
 * Example 1:
 *
 * Input: s = "leet**cod*e"
 * Output: "lecoe"
 * Explanation: Performing the removals from left to right:
 * - The closest character to the 1^st star is 't' in "lee<u>t</u>**cod*e". s becomes "lee*cod*e".
 * - The closest character to the 2^nd star is 'e' in "le<u>e</u>*cod*e". s becomes "lecod*e".
 * - The closest character to the 3^rd star is 'd' in "leco<u>d</u>*e". s becomes "lecoe".
 * There are no more stars, so we return "lecoe".
 * Example 2:
 *
 * Input: s = "erase*****"
 * Output: ""
 * Explanation: The entire string is removed, so we return an empty string.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters and stars *.
 * 	The operation above can be performed on s.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/removing-stars-from-a-string/
// discuss: https://leetcode.com/problems/removing-stars-from-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_stars(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2390_example_1() {
        let s = "leet**cod*e".to_string();

        let result = "lecoe".to_string();

        assert_eq!(Solution::remove_stars(s), result);
    }

    #[test]
    #[ignore]
    fn test_2390_example_2() {
        let s = "erase*****".to_string();

        let result = "".to_string();

        assert_eq!(Solution::remove_stars(s), result);
    }
}
