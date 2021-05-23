/**
 * [140] Word Break II
 *
 * Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * Example 1:
 *
 * Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
 * Output: ["cats and dog","cat sand dog"]
 *
 * Example 2:
 *
 * Input: s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
 * Output: ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
 * Explanation: Note that you are allowed to reuse a dictionary word.
 *
 * Example 3:
 *
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 20
 * 	1 <= wordDict.length <= 1000
 * 	1 <= wordDict[i].length <= 10
 * 	s and wordDict[i] consist of only lowercase English letters.
 * 	All the strings of wordDict are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-break-ii/
// discuss: https://leetcode.com/problems/word-break-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/word-break-ii/discuss/1120649/Naive-Rust-recursion-(but-works-%3A)
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn helper(ss: &[u8], dict: &[String]) -> Vec<String> {
            if ss.is_empty() {
                return vec!["".into()];
            }
            let mut ans = vec![];
            for word in dict {
                let len = word.len();
                if len > ss.len() {
                    continue;
                }
                if &ss[0..len] == word.as_bytes() {
                    helper(&ss[len..], dict)
                        .into_iter()
                        .map(|x| format!("{}{}{}", word, (if x == "" { "" } else { " " }), x))
                        .for_each(|s| ans.push(s));
                }
            }
            ans
        }

        let ss = s.as_bytes();
        helper(ss, &word_dict)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0140_example_1() {
        let s = "catsanddog".to_string();
        let word_dict = vec_string!["cat", "cats", "and", "sand", "dog"];
        let result = vec_string!["cats and dog", "cat sand dog"];

        assert_eq_sorted!(Solution::word_break(s, word_dict), result);
    }

    #[test]
    fn test_0140_example_2() {
        let s = "pineapplepenapple".to_string();
        let word_dict = vec_string!["apple", "pen", "applepen", "pine", "pineapple"];
        let result = vec_string![
            "pine apple pen apple",
            "pineapple pen apple",
            "pine applepen apple"
        ];

        assert_eq_sorted!(Solution::word_break(s, word_dict), result);
    }

    #[test]
    fn test_0140_example_3() {
        let s = "catsandog".to_string();
        let word_dict = vec_string!["cats", "dog", "sand", "and", "cat"];
        let result: Vec<String> = vec_string![];

        assert_eq_sorted!(Solution::word_break(s, word_dict), result);
    }
}
