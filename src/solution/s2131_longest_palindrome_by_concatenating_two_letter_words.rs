/**
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 *
 * You are given an array of strings words. Each element of words consists of two lowercase English letters.
 * Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
 * Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
 * A palindrome is a string that reads the same forward and backward.
 *  
 * Example 1:
 *
 * Input: words = ["lc","cl","gg"]
 * Output: 6
 * Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
 * Note that "clgglc" is another longest palindrome that can be created.
 *
 * Example 2:
 *
 * Input: words = ["ab","ty","yt","lc","cl","ab"]
 * Output: 8
 * Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
 * Note that "lcyttycl" is another longest palindrome that can be created.
 *
 * Example 3:
 *
 * Input: words = ["cc","ll","xx"]
 * Output: 2
 * Explanation: One longest palindrome is "cc", of length 2.
 * Note that "ll" is another longest palindrome that can be created, and so is "xx".
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 10^5
 * 	words[i].length == 2
 * 	words[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
// discuss: https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2131_example_1() {
        let words = vec_string!["lc", "cl", "gg"];

        let result = 6;

        assert_eq!(Solution::longest_palindrome(words), result);
    }

    #[test]
    #[ignore]
    fn test_2131_example_2() {
        let words = vec_string!["ab", "ty", "yt", "lc", "cl", "ab"];

        let result = 8;

        assert_eq!(Solution::longest_palindrome(words), result);
    }

    #[test]
    #[ignore]
    fn test_2131_example_3() {
        let words = vec_string!["cc", "ll", "xx"];

        let result = 2;

        assert_eq!(Solution::longest_palindrome(words), result);
    }
}
