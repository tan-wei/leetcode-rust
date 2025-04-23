/**
 * [1974] Minimum Time to Type Word Using Special Typewriter
 *
 * There is a special typewriter with lowercase English letters 'a' to 'z' arranged in a circle with a pointer. A character can only be typed if the pointer is pointing to that character. The pointer is initially pointing to the character 'a'.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/31/chart.jpg" style="width: 530px; height: 410px;" />
 * Each second, you may perform one of the following operations:
 *
 * 	Move the pointer one character counterclockwise or clockwise.
 * 	Type the character the pointer is currently on.
 *
 * Given a string word, return the minimum number of seconds to type out the characters in word.
 *  
 * Example 1:
 *
 * Input: word = "abc"
 * Output: 5
 * Explanation:
 * The characters are printed as follows:
 * - Type the character 'a' in 1 second since the pointer is initially on 'a'.
 * - Move the pointer clockwise to 'b' in 1 second.
 * - Type the character 'b' in 1 second.
 * - Move the pointer clockwise to 'c' in 1 second.
 * - Type the character 'c' in 1 second.
 *
 * Example 2:
 *
 * Input: word = "bza"
 * Output: 7
 * Explanation:
 * The characters are printed as follows:
 * - Move the pointer clockwise to 'b' in 1 second.
 * - Type the character 'b' in 1 second.
 * - Move the pointer counterclockwise to 'z' in 2 seconds.
 * - Type the character 'z' in 1 second.
 * - Move the pointer clockwise to 'a' in 1 second.
 * - Type the character 'a' in 1 second.
 *
 * Example 3:
 *
 * Input: word = "zjpc"
 * Output: 34
 * Explanation:
 * The characters are printed as follows:
 * - Move the pointer counterclockwise to 'z' in 1 second.
 * - Type the character 'z' in 1 second.
 * - Move the pointer clockwise to 'j' in 10 seconds.
 * - Type the character 'j' in 1 second.
 * - Move the pointer clockwise to 'p' in 6 seconds.
 * - Type the character 'p' in 1 second.
 * - Move the pointer counterclockwise to 'c' in 13 seconds.
 * - Type the character 'c' in 1 second.
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 100
 * 	word consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/
// discuss: https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        word.bytes()
            .fold((word.len() as i32, b'a'), |(time, pointer), b| {
                if pointer > b {
                    (time + (pointer - b).min(26 + b - pointer) as i32, b)
                } else {
                    (time + (b - pointer).min(26 + pointer - b) as i32, b)
                }
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1974_example_1() {
        let word = "abc".to_string();

        let result = 5;

        assert_eq!(Solution::min_time_to_type(word), result);
    }

    #[test]
    fn test_1974_example_2() {
        let word = "bza".to_string();

        let result = 7;

        assert_eq!(Solution::min_time_to_type(word), result);
    }

    #[test]
    fn test_1974_example_3() {
        let word = "zjpc".to_string();

        let result = 34;

        assert_eq!(Solution::min_time_to_type(word), result);
    }
}
