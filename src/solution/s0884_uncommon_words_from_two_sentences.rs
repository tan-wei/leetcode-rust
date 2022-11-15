/**
 * [0884] Uncommon Words from Two Sentences
 *
 * A sentence is a string of single-space separated words where each word consists only of lowercase letters.
 * A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
 * Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
 *  
 * Example 1:
 * Input: s1 = "this apple is sweet", s2 = "this apple is sour"
 * Output: ["sweet","sour"]
 * Example 2:
 * Input: s1 = "apple apple", s2 = "banana"
 * Output: ["banana"]
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 200
 * 	s1 and s2 consist of lowercase English letters and spaces.
 * 	s1 and s2 do not have leading or trailing spaces.
 * 	All the words in s1 and s2 are separated by a single space.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/uncommon-words-from-two-sentences/
// discuss: https://leetcode.com/problems/uncommon-words-from-two-sentences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split_whitespace()
            .chain(s2.split_whitespace())
            .fold(std::collections::HashMap::new(), |mut m, w| {
                let c = m.entry(w).or_insert(0usize);
                *c += 1;
                m
            })
            .iter()
            .fold(Vec::new(), |mut acc, (&s, &f)| {
                if f == 1 {
                    acc.push(s.to_owned())
                }
                acc
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0884_example_1() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let result = vec_string!["sweet", "sour"];

        assert_eq_sorted!(Solution::uncommon_from_sentences(s1, s2), result);
    }

    #[test]
    fn test_0884_example_2() {
        let s1 = "apple apple".to_string();
        let s2 = "banana".to_string();
        let result = vec_string!["banana"];

        assert_eq_sorted!(Solution::uncommon_from_sentences(s1, s2), result);
    }
}
