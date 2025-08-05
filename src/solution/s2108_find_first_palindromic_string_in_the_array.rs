/**
 * [2108] Find First Palindromic String in the Array
 *
 * Given an array of strings words, return the first palindromic string in the array. If there is no such string, return an empty string "".
 * A string is palindromic if it reads the same forward and backward.
 *  
 * Example 1:
 *
 * Input: words = ["abc","car","ada","racecar","cool"]
 * Output: "ada"
 * Explanation: The first string that is palindromic is "ada".
 * Note that "racecar" is also palindromic, but it is not the first.
 *
 * Example 2:
 *
 * Input: words = ["notapalindrome","racecar"]
 * Output: "racecar"
 * Explanation: The first and only string that is palindromic is "racecar".
 *
 * Example 3:
 *
 * Input: words = ["def","ghi"]
 * Output: ""
 * Explanation: There are no palindromic strings, so the empty string is returned.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 100
 * 	words[i] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// discuss: https://leetcode.com/problems/find-first-palindromic-string-in-the-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| w.chars().eq(w.chars().rev()))
            .unwrap_or_else(String::default)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2108_example_1() {
        let words = vec_string!["abc", "car", "ada", "racecar", "cool"];

        let result = "ada".to_string();

        assert_eq!(Solution::first_palindrome(words), result);
    }

    #[test]
    fn test_2108_example_2() {
        let words = vec_string!["notapalindrome", "racecar"];

        let result = "racecar".to_string();

        assert_eq!(Solution::first_palindrome(words), result);
    }

    #[test]
    fn test_2108_example_3() {
        let words = vec_string!["def", "ghi"];

        let result = "".to_string();

        assert_eq!(Solution::first_palindrome(words), result);
    }
}
