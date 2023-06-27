/**
 * [1178] Number of Valid Words for Each Puzzle
 *
 * With respect to a given puzzle string, a word is valid if both the following conditions are satisfied:
 * 	word contains the first letter of puzzle.
 * 	For each letter in word, that letter is in puzzle.
 *
 * 		For example, if the puzzle is "abcdefg", then valid words are "faced", "cabbage", and "baggage", while
 * 		invalid words are "beefed" (does not include 'a') and "based" (includes 's' which is not in the puzzle).
 *
 *
 * Return an array answer, where answer[i] is the number of words in the given word list words that is valid with respect to the puzzle puzzles[i].
 *  
 * Example 1:
 *
 * Input: words = ["aaaa","asas","able","ability","actt","actor","access"], puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
 * Output: [1,1,3,2,4,0]
 * Explanation:
 * 1 valid word for "aboveyz" : "aaaa"
 * 1 valid word for "abrodyz" : "aaaa"
 * 3 valid words for "abslute" : "aaaa", "asas", "able"
 * 2 valid words for "absoryz" : "aaaa", "asas"
 * 4 valid words for "actresz" : "aaaa", "asas", "actt", "access"
 * There are no valid words for "gaswxyz" cause none of the words in the list contains letter 'g'.
 *
 * Example 2:
 *
 * Input: words = ["apple","pleas","please"], puzzles = ["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"]
 * Output: [0,1,3,2,0]
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 10^5
 * 	4 <= words[i].length <= 50
 * 	1 <= puzzles.length <= 10^4
 * 	puzzles[i].length == 7
 * 	words[i] and puzzles[i] consist of lowercase English letters.
 * 	Each puzzles[i] does not contain repeated characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/
// discuss: https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solutions/1529729/rust-using-simple-bitwise-and/
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut words = words
            .iter()
            .map(|w| Self::word_to_int(w))
            .collect::<Vec<_>>();
        let mut puzzles = puzzles
            .iter()
            .map(|w| {
                let as_int = Self::word_to_int(w);
                let first_ch = Self::first_char_to_mask(w);
                (as_int, first_ch)
            })
            .collect::<Vec<_>>();

        let mut result = Vec::with_capacity(puzzles.len());
        for &(puzzle, first_ch) in puzzles.iter() {
            let mut counter = 0;
            for &word in words.iter() {
                if (word & puzzle == word) & (word & first_ch == first_ch) {
                    counter += 1;
                }
            }
            result.push(counter);
        }
        result
    }

    fn word_to_int<W: AsRef<str>>(word: W) -> u32 {
        let word = word.as_ref().as_bytes();
        let mut bits = 0;

        for &ch in word {
            bits |= 1 << ch - b'a';
        }

        bits
    }

    fn first_char_to_mask<W: AsRef<str>>(word: W) -> u32 {
        let word = word.as_ref().as_bytes();
        1 << word[0] - b'a'
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1178_example_1() {
        let words = vec_string!["aaaa", "asas", "able", "ability", "actt", "actor", "access"];
        let puzzles = vec_string!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"];
        let result = vec![1, 1, 3, 2, 4, 0];

        assert_eq!(Solution::find_num_of_valid_words(words, puzzles), result);
    }

    #[test]
    fn test_1178_example_2() {
        let words = vec_string!["apple", "pleas", "please"];
        let puzzles = vec_string!["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"];
        let result = vec![0, 1, 3, 2, 0];

        assert_eq!(Solution::find_num_of_valid_words(words, puzzles), result);
    }
}
