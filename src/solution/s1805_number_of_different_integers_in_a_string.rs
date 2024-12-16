/**
 * [1805] Number of Different Integers in a String
 *
 * You are given a string word that consists of digits and lowercase English letters.
 * You will replace every non-digit character with a space. For example, "a123bc34d8ef34" will become " 123  34 8  34". Notice that you are left with some integers that are separated by at least one space: "123", "34", "8", and "34".
 * Return the number of different integers after performing the replacement operations on word.
 * Two integers are considered different if their decimal representations without any leading zeros are different.
 *  
 * Example 1:
 *
 * Input: word = "a<u>123</u>bc<u>34</u>d<u>8</u>ef<u>34</u>"
 * Output: 3
 * Explanation: The three different integers are "123", "34", and "8". Notice that "34" is only counted once.
 *
 * Example 2:
 *
 * Input: word = "leet<u>1234</u>code<u>234</u>"
 * Output: 2
 *
 * Example 3:
 *
 * Input: word = "a<u>1</u>b<u>01</u>c<u>001</u>"
 * Output: 1
 * Explanation: The three integers "1", "01", and "001" all represent the same integer because
 * the leading zeros are ignored when comparing their decimal values.
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 1000
 * 	word consists of digits and lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-different-integers-in-a-string/
// discuss: https://leetcode.com/problems/number-of-different-integers-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(char::is_alphabetic)
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('0'))
            .collect::<std::collections::HashSet<_>>()
            .len() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1805_example_1() {
        let word = "a123bc34d8ef34".to_string();

        let result = 3;

        assert_eq!(Solution::num_different_integers(word), result);
    }

    #[test]
    fn test_1805_example_2() {
        let word = "leet1234code234".to_string();

        let result = 2;

        assert_eq!(Solution::num_different_integers(word), result);
    }

    #[test]
    fn test_1805_example_3() {
        let word = "a1b01c001".to_string();

        let result = 1;

        assert_eq!(Solution::num_different_integers(word), result);
    }
}
