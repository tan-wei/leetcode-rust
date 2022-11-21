/**
 * [0890] Find and Replace Pattern
 *
 * Given a list of strings words and a string pattern, return a list of words[i] that match pattern. You may return the answer in any order.
 * A word matches the pattern if there exists a permutation of letters p so that after replacing every letter x in the pattern with p(x), we get the desired word.
 * Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.
 *  
 * Example 1:
 *
 * Input: words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
 * Output: ["mee","aqq"]
 * Explanation: "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
 * "ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
 *
 * Example 2:
 *
 * Input: words = ["a","b","c"], pattern = "a"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 * 	1 <= pattern.length <= 20
 * 	1 <= words.length <= 50
 * 	words[i].length == pattern.length
 * 	pattern and words[i] are lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-and-replace-pattern/
// discuss: https://leetcode.com/problems/find-and-replace-pattern/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| {
                let mut pmap = vec!['\0'; 26];
                word.chars()
                    .zip(pattern.chars().map(|p| p as usize - 'a' as usize))
                    .all(|(c, i)| {
                        if pmap[i] == '\0' {
                            pmap[i] = c;
                            pmap.iter().filter(|&&c| c == pmap[i]).count() == 1
                        } else {
                            c == pmap[i]
                        }
                    })
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0890_example_1() {
        let words = vec_string!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
        let pattern = "abb".to_string();
        let result = vec_string!["mee", "aqq"];

        assert_eq!(Solution::find_and_replace_pattern(words, pattern), result);
    }

    #[test]
    fn test_0890_example_2() {
        let words = vec_string!["a", "b", "c"];
        let pattern = "a".to_string();
        let result = vec_string!["a", "b", "c"];

        assert_eq!(Solution::find_and_replace_pattern(words, pattern), result);
    }
}
