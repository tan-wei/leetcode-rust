/**
 * [1002] Find Common Characters
 *
 * Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.
 *  
 * Example 1:
 * Input: words = ["bella","label","roller"]
 * Output: ["e","l","l"]
 * Example 2:
 * Input: words = ["cool","lock","cook"]
 * Output: ["c","o"]
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 100
 * 	words[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-common-characters/
// discuss: https://leetcode.com/problems/find-common-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let occur =
            words
                .iter()
                .enumerate()
                .fold(vec![vec![0; words.len()]; 26], |mut acc, (i, s)| {
                    s.bytes().for_each(|c| {
                        acc[(c - b'a') as usize][i] += 1;
                    });
                    acc
                });

        occur
            .iter()
            .map(|v| v.iter().min().unwrap())
            .enumerate()
            .filter(|(i, c)| **c > 0)
            .map(|(i, c)| [(b'a' + (i as u8)) as char].repeat(*c))
            .flatten()
            .map(|c| c.to_string())
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1002_example_1() {
        let words = vec_string!["bella", "label", "roller"];
        let result = vec_string!["e", "l", "l"];

        assert_eq_sorted!(Solution::common_chars(words), result);
    }

    #[test]
    fn test_1002_example_2() {
        let words = vec_string!["cool", "lock", "cook"];
        let result = vec_string!["c", "o"];

        assert_eq_sorted!(Solution::common_chars(words), result);
    }
}
