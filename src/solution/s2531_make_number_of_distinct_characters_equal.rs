/**
 * [2531] Make Number of Distinct Characters Equal
 *
 * You are given two 0-indexed strings word1 and word2.
 * A move consists of choosing two indices i and j such that 0 <= i < word1.length and 0 <= j < word2.length and swapping word1[i] with word2[j].
 * Return true if it is possible to get the number of distinct characters in word1 and word2 to be equal with exactly one move. Return false otherwise.
 *  
 * Example 1:
 *
 * Input: word1 = "ac", word2 = "b"
 * Output: false
 * Explanation: Any pair of swaps would yield two distinct characters in the first string, and one in the second string.
 *
 * Example 2:
 *
 * Input: word1 = "abcc", word2 = "aab"
 * Output: true
 * Explanation: We swap index 2 of the first string with index 0 of the second string. The resulting strings are word1 = "abac" and word2 = "cab", which both have 3 distinct characters.
 *
 * Example 3:
 *
 * Input: word1 = "abcde", word2 = "fghij"
 * Output: true
 * Explanation: Both resulting strings will have 5 distinct characters, regardless of which indices we swap.
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 10^5
 * 	word1 and word2 consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-number-of-distinct-characters-equal/
// discuss: https://leetcode.com/problems/make-number-of-distinct-characters-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2531_example_1() {
        let word1 = "ac".to_string();
        let word2 = "b".to_string();

        let result = false;

        assert_eq!(Solution::is_it_possible(word1, word2), result);
    }

    #[test]
    #[ignore]
    fn test_2531_example_2() {
        let word1 = "abcc".to_string();
        let word2 = "aab".to_string();

        let result = true;

        assert_eq!(Solution::is_it_possible(word1, word2), result);
    }

    #[test]
    #[ignore]
    fn test_2531_example_3() {
        let word1 = "abcde".to_string();
        let word2 = "fghij".to_string();

        let result = true;

        assert_eq!(Solution::is_it_possible(word1, word2), result);
    }
}
