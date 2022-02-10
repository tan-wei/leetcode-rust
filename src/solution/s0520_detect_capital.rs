/**
 * [0520] Detect Capital
 *
 * We define the usage of capitals in a word to be right when one of the following cases holds:
 *
 * 	All letters in this word are capitals, like "USA".
 * 	All letters in this word are not capitals, like "leetcode".
 * 	Only the first letter in this word is capital, like "Google".
 *
 * Given a string word, return true if the usage of capitals in it is right.
 *  
 * Example 1:
 * Input: word = "USA"
 * Output: true
 * Example 2:
 * Input: word = "FlaG"
 * Output: false
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 100
 * 	word consists of lowercase and uppercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/detect-capital/
// discuss: https://leetcode.com/problems/detect-capital/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().nth(0).unwrap().is_uppercase()
            && (word.chars().skip(1).all(|x| x.is_uppercase())
                || word.chars().skip(1).all(|x| x.is_lowercase()))
            || word.chars().all(|x| x.is_lowercase())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0520_example_1() {
        let word = "USA".to_string();
        let result = true;

        assert_eq!(Solution::detect_capital_use(word), result);
    }

    #[test]
    fn test_0520_example_2() {
        let word = "FlaG".to_string();
        let result = false;

        assert_eq!(Solution::detect_capital_use(word), result);
    }

    #[test]
    fn test_0520_additional_case_1() {
        let word = "usa".to_string();
        let result = true;

        assert_eq!(Solution::detect_capital_use(word), result);
    }

    #[test]
    fn test_0520_additional_case_2() {
        let word = "Google".to_string();
        let result = true;

        assert_eq!(Solution::detect_capital_use(word), result);
    }
}
