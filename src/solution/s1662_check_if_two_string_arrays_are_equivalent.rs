/**
 * [1662] Check If Two String Arrays are Equivalent
 *
 * Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
 * A string is represented by an array if the array elements concatenated in order forms the string.
 *  
 * Example 1:
 *
 * Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
 * Output: true
 * Explanation:
 * word1 represents string "ab" + "c" -> "abc"
 * word2 represents string "a" + "bc" -> "abc"
 * The strings are the same, so return true.
 * Example 2:
 *
 * Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
 * Output: false
 *
 * Example 3:
 *
 * Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 10^3
 * 	1 <= word1[i].length, word2[i].length <= 10^3
 * 	1 <= sum(word1[i].length), sum(word2[i].length) <= 10^3
 * 	word1[i] and word2[i] consist of lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
// discuss: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1662_example_1() {
        let word1 = vec_string!["ab", "c"];
        let word2 = vec_string!["a", "bc"];

        let result = true;

        assert_eq!(Solution::array_strings_are_equal(word1, word2), result);
    }

    #[test]
    fn test_1662_example_2() {
        let word1 = vec_string!["a", "cb"];
        let word2 = vec_string!["ab", "c"];

        let result = false;

        assert_eq!(Solution::array_strings_are_equal(word1, word2), result);
    }

    #[test]
    fn test_1662_example_3() {
        let word1 = vec_string!["abc", "d", "defg"];
        let word2 = vec_string!["abcddefg"];

        let result = true;

        assert_eq!(Solution::array_strings_are_equal(word1, word2), result);
    }
}
