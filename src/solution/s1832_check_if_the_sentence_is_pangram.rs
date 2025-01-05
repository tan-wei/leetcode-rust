/**
 * [1832] Check if the Sentence Is Pangram
 *
 * A pangram is a sentence where every letter of the English alphabet appears at least once.
 * Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.
 *  
 * Example 1:
 *
 * Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
 * Output: true
 * Explanation: sentence contains at least one of every letter of the English alphabet.
 *
 * Example 2:
 *
 * Input: sentence = "leetcode"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= sentence.length <= 1000
 * 	sentence consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-the-sentence-is-pangram/
// discuss: https://leetcode.com/problems/check-if-the-sentence-is-pangram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence
            .bytes()
            .scan(0_u32, |bitset, b| {
                *bitset |= 1 << (b - b'a');
                Some(*bitset)
            })
            .any(|bitset| bitset == (1 << N_LETTERS) - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1832_example_1() {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();

        let result = true;

        assert_eq!(Solution::check_if_pangram(sentence), result);
    }

    #[test]
    fn test_1832_example_2() {
        let sentence = "leetcode".to_string();

        let result = false;

        assert_eq!(Solution::check_if_pangram(sentence), result);
    }
}
