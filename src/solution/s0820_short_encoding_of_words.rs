/**
 * [0820] Short Encoding of Words
 *
 * A valid encoding of an array of words is any reference string s and array of indices indices such that:
 *
 * 	words.length == indices.length
 * 	The reference string s ends with the '#' character.
 * 	For each index indices[i], the substring of s starting from indices[i] and up to (but not including) the next '#' character is equal to words[i].
 *
 * Given an array of words, return the length of the shortest reference string s possible of any valid encoding of words.
 *  
 * Example 1:
 *
 * Input: words = ["time", "me", "bell"]
 * Output: 10
 * Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
 * words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#' is underlined in "<u>time</u>#bell#"
 * words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#' is underlined in "ti<u>me</u>#bell#"
 * words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#' is underlined in "time#<u>bell</u>#"
 *
 * Example 2:
 *
 * Input: words = ["t"]
 * Output: 2
 * Explanation: A valid encoding would be s = "t#" and indices = [0].
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 2000
 * 	1 <= words[i].length <= 7
 * 	words[i] consists of only lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/short-encoding-of-words/
// discuss: https://leetcode.com/problems/short-encoding-of-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut ignore: std::collections::HashSet<_> = words
            .iter()
            .flat_map(|w| w.char_indices().skip(1).map(move |(i, _)| &w[i..]))
            .collect();
        words
            .iter()
            .filter_map(|w| Some(w.len() as i32 + 1).filter(|_| ignore.insert(w)))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0820_example_1() {
        let words = vec_string!["time", "me", "bell"];
        let result = 10;

        assert_eq!(Solution::minimum_length_encoding(words), result);
    }

    #[test]
    fn test_0820_example_2() {
        let words = vec_string!["t"];
        let result = 2;

        assert_eq!(Solution::minimum_length_encoding(words), result);
    }
}
