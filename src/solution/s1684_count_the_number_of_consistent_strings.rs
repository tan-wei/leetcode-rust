/**
 * [1684] Count the Number of Consistent Strings
 *
 * You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.
 * Return the number of consistent strings in the array words.
 *  
 * Example 1:
 *
 * Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
 * Output: 2
 * Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
 *
 * Example 2:
 *
 * Input: allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
 * Output: 7
 * Explanation: All strings are consistent.
 *
 * Example 3:
 *
 * Input: allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
 * Output: 4
 * Explanation: Strings "cc", "acd", "ac", and "d" are consistent.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 10^4
 * 	1 <= allowed.length <=^ 26
 * 	1 <= words[i].length <= 10
 * 	The characters in allowed are distinct.
 * 	words[i] and allowed contain only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-consistent-strings/
// discuss: https://leetcode.com/problems/count-the-number-of-consistent-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let set = allowed.chars().collect::<std::collections::HashSet<_>>();
        words
            .into_iter()
            .filter(|w| w.chars().all(|c| set.contains(&c)))
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1684_example_1() {
        let allowed = "ab".to_string();
        let words = vec_string!["ad", "bd", "aaab", "baa", "badab"];

        let result = 2;

        assert_eq!(Solution::count_consistent_strings(allowed, words), result);
    }

    #[test]
    fn test_1684_example_2() {
        let allowed = "abc".to_string();
        let words = vec_string!["a", "b", "c", "ab", "ac", "bc", "abc"];

        let result = 7;

        assert_eq!(Solution::count_consistent_strings(allowed, words), result);
    }

    #[test]
    fn test_1684_example_3() {
        let allowed = "cad".to_string();
        let words = vec_string!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"];

        let result = 4;

        assert_eq!(Solution::count_consistent_strings(allowed, words), result);
    }
}
