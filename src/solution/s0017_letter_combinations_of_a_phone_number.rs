/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" style="width: 200px; height: 162px;" />
 *  
 * Example 1:
 *
 * Input: digits = "23"
 * Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * Example 2:
 *
 * Input: digits = ""
 * Output: []
 *
 * Example 3:
 *
 * Input: digits = "2"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 * 	0 <= digits.length <= 4
 * 	digits[i] is a digit in the range ['2', '9'].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// discuss: https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    const DIGIT_TO_CHAR: [&'static str; 10] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut letters = digits.chars();
        let head = letters.next().unwrap().to_string();
        let tail = Solution::letter_combinations(letters.collect());

        if tail.is_empty() {
            Solution::DIGIT_TO_CHAR[head.parse::<usize>().unwrap()]
                .chars()
                .map(|letter| letter.to_string())
                .collect()
        } else if head == "1" {
            tail
        } else {
            Solution::DIGIT_TO_CHAR[head.parse::<usize>().unwrap()]
                .chars()
                .map(|prefix| {
                    tail.iter()
                        .map(|suffix| {
                            std::iter::once(prefix)
                                .chain(suffix.chars())
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                })
                .flatten()
                .collect()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0017_exmaple_1() {
        let digits = "23".to_string();
        let result = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

        assert_eq!(Solution::letter_combinations(digits), result);
    }

    #[test]
    fn test_0017_exmaple_2() {
        let digits = "".to_string();
        let result: Vec<String> = vec![];

        assert_eq!(Solution::letter_combinations(digits), result);
    }

    #[test]
    fn test_0017_exmaple_3() {
        let digits = "2".to_string();
        let result = vec!["a", "b", "c"];

        assert_eq!(Solution::letter_combinations(digits), result);
    }
}
