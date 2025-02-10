/**
 * [1880] Check if Word Equals Summation of Two Words
 *
 * The letter value of a letter is its position in the alphabet starting from 0 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, etc.).
 * The numerical value of some string of lowercase English letters s is the concatenation of the letter values of each letter in s, which is then converted into an integer.
 *
 * 	For example, if s = "acb", we concatenate each letter's letter value, resulting in "021". After converting it, we get 21.
 *
 * You are given three strings firstWord, secondWord, and targetWord, each consisting of lowercase English letters 'a' through 'j' inclusive.
 * Return true if the summation of the numerical values of firstWord and secondWord equals the numerical value of targetWord, or false otherwise.
 *  
 * Example 1:
 *
 * Input: firstWord = "acb", secondWord = "cba", targetWord = "cdb"
 * Output: true
 * Explanation:
 * The numerical value of firstWord is "acb" -> "021" -> 21.
 * The numerical value of secondWord is "cba" -> "210" -> 210.
 * The numerical value of targetWord is "cdb" -> "231" -> 231.
 * We return true because 21 + 210 == 231.
 *
 * Example 2:
 *
 * Input: firstWord = "aaa", secondWord = "a", targetWord = "aab"
 * Output: false
 * Explanation:
 * The numerical value of firstWord is "aaa" -> "000" -> 0.
 * The numerical value of secondWord is "a" -> "0" -> 0.
 * The numerical value of targetWord is "aab" -> "001" -> 1.
 * We return false because 0 + 0 != 1.
 *
 * Example 3:
 *
 * Input: firstWord = "aaa", secondWord = "a", targetWord = "aaaa"
 * Output: true
 * Explanation:
 * The numerical value of firstWord is "aaa" -> "000" -> 0.
 * The numerical value of secondWord is "a" -> "0" -> 0.
 * The numerical value of targetWord is "aaaa" -> "0000" -> 0.
 * We return true because 0 + 0 == 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= firstWord.length, secondWord.length, targetWord.length <= 8
 * 	firstWord, secondWord, and targetWord consist of lowercase English letters from 'a' to 'j' inclusive.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
// discuss: https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::to_number(first_word) + Self::to_number(second_word) == Self::to_number(target_word)
    }

    fn to_number(word: String) -> u8 {
        let mut num: u8 = 0;
        for ch in word.chars() {
            num = num * 10 + (ch as u8 - 'a' as u8);
        }
        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1880_example_1() {
        let first_word = "acb".to_string();
        let second_word = "cba".to_string();
        let target_word = "cdb".to_string();

        let result = true;

        assert_eq!(
            Solution::is_sum_equal(first_word, second_word, target_word),
            result
        );
    }

    #[test]
    fn test_1880_example_2() {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aab".to_string();

        let result = false;

        assert_eq!(
            Solution::is_sum_equal(first_word, second_word, target_word),
            result
        );
    }

    #[test]
    fn test_1880_example_() {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aaaa".to_string();

        let result = true;

        assert_eq!(
            Solution::is_sum_equal(first_word, second_word, target_word),
            result
        );
    }
}
