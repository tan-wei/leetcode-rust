/**
 * [2042] Check if Numbers Are Ascending in a Sentence
 *
 * A sentence is a list of tokens separated by a single space with no leading or trailing spaces. Every token is either a positive number consisting of digits 0-9 with no leading zeros, or a word consisting of lowercase English letters.
 *
 * 	For example, "a puppy has 2 eyes 4 legs" is a sentence with seven tokens: "2" and "4" are numbers and the other tokens such as "puppy" are words.
 *
 * Given a string s representing a sentence, you need to check if all the numbers in s are strictly increasing from left to right (i.e., other than the last number, each number is strictly smaller than the number on its right in s).
 * Return true if so, or false otherwise.
 *  
 * Example 1:
 * <img alt="example-1" src="https://assets.leetcode.com/uploads/2021/09/30/example1.png" style="width: 637px; height: 48px;" />
 * Input: s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
 * Output: true
 * Explanation: The numbers in s are: 1, 3, 4, 6, 12.
 * They are strictly increasing from left to right: 1 < 3 < 4 < 6 < 12.
 *
 * Example 2:
 *
 * Input: s = "hello world 5 x 5"
 * Output: false
 * Explanation: The numbers in s are: <u>5</u>, <u>5</u>. They are not strictly increasing.
 *
 * Example 3:
 * <img alt="example-3" src="https://assets.leetcode.com/uploads/2021/09/30/example3.png" style="width: 794px; height: 48px;" />
 * Input: s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
 * Output: false
 * Explanation: The numbers in s are: 7, <u>51</u>, <u>50</u>, 60. They are not strictly increasing.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 200
 * 	s consists of lowercase English letters, spaces, and digits from 0 to 9, inclusive.
 * 	The number of tokens in s is between 2 and 100, inclusive.
 * 	The tokens in s are separated by a single space.
 * 	There are at least two numbers in s.
 * 	Each number in s is a positive number less than 100, with no leading zeros.
 * 	s contains no leading or trailing spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
// discuss: https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        s.split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok())
            .scan(None, |maybe_prev, curr| {
                let output = if let Some(prev) = maybe_prev {
                    (*prev) < curr
                } else {
                    true
                };
                *maybe_prev = Some(curr);
                Some(output)
            })
            .all(std::convert::identity)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2042_example_1() {
        let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();

        let result = true;

        assert_eq!(Solution::are_numbers_ascending(s), result);
    }

    #[test]
    fn test_2042_example_2() {
        let s = "hello world 5 x 5".to_string();

        let result = false;

        assert_eq!(Solution::are_numbers_ascending(s), result);
    }

    #[test]
    fn test_2042_example_3() {
        let s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string();

        let result = false;

        assert_eq!(Solution::are_numbers_ascending(s), result);
    }
}
