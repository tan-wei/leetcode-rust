/**
 * [2085] Count Common Words With One Occurrence
 *
 * Given two string arrays words1 and words2, return the number of strings that appear exactly once in each of the two arrays.
 *  
 * Example 1:
 *
 * Input: words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
 * Output: 2
 * Explanation:
 * - "leetcode" appears exactly once in each of the two arrays. We count this string.
 * - "amazing" appears exactly once in each of the two arrays. We count this string.
 * - "is" appears in each of the two arrays, but there are 2 occurrences of it in words1. We do not count this string.
 * - "as" appears once in words1, but does not appear in words2. We do not count this string.
 * Thus, there are 2 strings that appear exactly once in each of the two arrays.
 *
 * Example 2:
 *
 * Input: words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
 * Output: 0
 * Explanation: There are no strings that appear in each of the two arrays.
 *
 * Example 3:
 *
 * Input: words1 = ["a","ab"], words2 = ["a","a","a","ab"]
 * Output: 1
 * Explanation: The only string that appears exactly once in each of the two arrays is "ab".
 *
 *  
 * Constraints:
 *
 * 	1 <= words1.length, words2.length <= 1000
 * 	1 <= words1[i].length, words2[j].length <= 30
 * 	words1[i] and words2[j] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-common-words-with-one-occurrence/
// discuss: https://leetcode.com/problems/count-common-words-with-one-occurrence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        words1.iter().for_each(|w| *hm.entry(w).or_insert(0) += 1);
        words2.iter().for_each(|w| {
            hm.entry(w).and_modify(|v| {
                if *v < 2 {
                    *v -= 1
                }
            });
        });
        hm.values().filter(|v| **v == 0).count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2085_example_1() {
        let words1 = vec_string!["leetcode", "is", "amazing", "as", "is"];
        let words2 = vec_string!["amazing", "leetcode", "is"];

        let result = 2;

        assert_eq!(Solution::count_words(words1, words2), result);
    }

    #[test]
    fn test_2085_example_2() {
        let words1 = vec_string!["b", "bb", "bbb"];
        let words2 = vec_string!["a", "aa", "aaa"];

        let result = 0;

        assert_eq!(Solution::count_words(words1, words2), result);
    }

    #[test]
    fn test_2085_example_3() {
        let words1 = vec_string!["a", "ab"];
        let words2 = vec_string!["a", "a", "a", "ab"];

        let result = 1;

        assert_eq!(Solution::count_words(words1, words2), result);
    }
}
