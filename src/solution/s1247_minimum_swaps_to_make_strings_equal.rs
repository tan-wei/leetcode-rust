/**
 * [1247] Minimum Swaps to Make Strings Equal
 *
 * You are given two strings s1 and s2 of equal length consisting of letters "x" and "y" only. Your task is to make these two strings equal to each other. You can swap any two characters that belong to different strings, which means: swap s1[i] and s2[j].
 * Return the minimum number of swaps required to make s1 and s2 equal, or return -1 if it is impossible to do so.
 *  
 * Example 1:
 *
 * Input: s1 = "xx", s2 = "yy"
 * Output: 1
 * Explanation: Swap s1[0] and s2[1], s1 = "yx", s2 = "yx".
 *
 * Example 2:
 *
 * Input: s1 = "xy", s2 = "yx"
 * Output: 2
 * Explanation: Swap s1[0] and s2[0], s1 = "yy", s2 = "xx".
 * Swap s1[0] and s2[1], s1 = "xy", s2 = "xy".
 * Note that you cannot swap s1[0] and s1[1] to make s1 equal to "yx", cause we can only swap chars in different strings.
 *
 * Example 3:
 *
 * Input: s1 = "xx", s2 = "xy"
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 1000
 * 	s1.length == s2.length
 * 	s1, s2 only contain 'x' or 'y'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/
// discuss: https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut xy = 0;
        let mut yx = 0;

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == 'x' && c2 == 'y' {
                xy += 1;
            }
            if c1 == 'y' && c2 == 'x' {
                yx += 1;
            }
        }

        let rxy = xy % 2;
        let ryx = yx % 2;

        if rxy != ryx {
            return -1;
        }

        yx / 2 + xy / 2 + rxy * 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1247_example_1() {
        let s1 = "xx".to_string();
        let s2 = "yy".to_string();
        let result = 1;

        assert_eq!(Solution::minimum_swap(s1, s2), result);
    }

    #[test]
    fn test_1247_example_2() {
        let s1 = "xy".to_string();
        let s2 = "yx".to_string();
        let result = 2;

        assert_eq!(Solution::minimum_swap(s1, s2), result);
    }

    #[test]
    fn test_1247_example_3() {
        let s1 = "xx".to_string();
        let s2 = "xy".to_string();
        let result = -1;

        assert_eq!(Solution::minimum_swap(s1, s2), result);
    }
}
