/**
 * [1864] Minimum Number of Swaps to Make the Binary String Alternating
 *
 * Given a binary string s, return the minimum number of character swaps to make it alternating, or -1 if it is impossible.
 * The string is called alternating if no two adjacent characters are equal. For example, the strings "010" and "1010" are alternating, while the string "0100" is not.
 * Any two characters may be swapped, even if they are not adjacent.
 *  
 * Example 1:
 *
 * Input: s = "111000"
 * Output: 1
 * Explanation: Swap positions 1 and 4: "1<u>1</u>10<u>0</u>0" -> "1<u>0</u>10<u>1</u>0"
 * The string is now alternating.
 *
 * Example 2:
 *
 * Input: s = "010"
 * Output: 0
 * Explanation: The string is already alternating, no swaps are needed.
 *
 * Example 3:
 *
 * Input: s = "1110"
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-binary-string-alternating/
// discuss: https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-binary-string-alternating/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1864_example_1() {
        let s = "111000".to_string();

        let result = 1;

        assert_eq!(Solution::min_swaps(s), result);
    }

    #[test]
    #[ignore]
    fn test_1864_example_2() {
        let s = "010".to_string();

        let result = 0;

        assert_eq!(Solution::min_swaps(s), result);
    }

    #[test]
    #[ignore]
    fn test_1864_example_3() {
        let s = "1110".to_string();

        let result = -1;

        assert_eq!(Solution::min_swaps(s), result);
    }
}
