/**
 * [139] Word Break
 *
 * Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * Example 1:
 *
 * Input:
 * s = "leetcode", wordDict = ["leet","code"]
 * Output: true
 * Explanation: Return true because "leetcode" can be segmented as "leet code".
 *
 * Example 2:
 *
 * Input: s = "applepenapple", wordDict = ["apple","pen"]
 * Output: true
 * Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
 * Note that you are allowed to reuse a dictionary word.
 *
 * Example 3:
 *
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 300
 * 	1 <= wordDict.length <= 1000
 * 	1 <= wordDict[i].length <= 20
 * 	s and wordDict[i] consist of only lowercase English letters.
 * 	All the strings of wordDict are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-break/
// discuss: https://leetcode.com/problems/word-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: std::collections::HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp: Vec<bool> = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in (0..=i - 1).rev() {
                if dp[j] && dict.contains(&s[j..=i - 1]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0139_example_1() {
        let s = "leetcode".to_string();
        let word_dict = vec_string!["leet", "code"];
        let result = true;

        assert_eq!(Solution::word_break(s, word_dict), result);
    }

    #[test]
    fn test_0139_example_2() {
        let s = "catsandog".to_string();
        let word_dict = vec_string!["cats", "dog", "sand", "and", "cat"];
        let result = false;

        assert_eq!(Solution::word_break(s, word_dict), result);
    }
}
