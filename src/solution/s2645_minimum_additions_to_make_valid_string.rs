/**
 * [2645] Minimum Additions to Make Valid String
 *
 * Given a string word to which you can insert letters "a", "b" or "c" anywhere and any number of times, return the minimum number of letters that must be inserted so that word becomes valid.
 * A string is called valid if it can be formed by concatenating the string "abc" several times.
 *  
 * Example 1:
 *
 * Input: word = "b"
 * Output: 2
 * Explanation: Insert the letter "a" right before "b", and the letter "c" right next to "b" to obtain the valid string "abc".
 *
 * Example 2:
 *
 * Input: word = "aaa"
 * Output: 6
 * Explanation: Insert letters "b" and "c" next to each "a" to obtain the valid string "abcabcabc".
 *
 * Example 3:
 *
 * Input: word = "abc"
 * Output: 0
 * Explanation: word is already valid. No modifications are needed.
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 50
 * 	word consists of letters "a", "b" and "c" only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-additions-to-make-valid-string/
// discuss: https://leetcode.com/problems/minimum-additions-to-make-valid-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        word.chars()
            .chain("a".chars())
            .fold(('c', 0), |(c, cnt), x| match (c, x) {
                ('a', 'b') | ('b', 'c') | ('c', 'a') => (x, cnt),
                ('a', 'c') | ('b', 'a') | ('c', 'b') => (x, cnt + 1),
                ('a', 'a') | ('b', 'b') | ('c', 'c') => (x, cnt + 2),
                _ => (x, cnt),
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2645_example_1() {
        let word = "b".to_string();

        let result = 2;

        assert_eq!(Solution::add_minimum(word), result);
    }

    #[test]
    fn test_2645_example_2() {
        let word = "aaa".to_string();

        let result = 6;

        assert_eq!(Solution::add_minimum(word), result);
    }

    #[test]
    fn test_2645_example_3() {
        let word = "abc".to_string();

        let result = 0;

        assert_eq!(Solution::add_minimum(word), result);
    }
}
