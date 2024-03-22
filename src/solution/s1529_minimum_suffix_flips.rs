/**
 * [1529] Minimum Suffix Flips
 *
 * You are given a 0-indexed binary string target of length n. You have another binary string s of length n that is initially set to all zeros. You want to make s equal to target.
 * In one operation, you can pick an index i where 0 <= i < n and flip all bits in the inclusive range [i, n - 1]. Flip means changing '0' to '1' and '1' to '0'.
 * Return the minimum number of operations needed to make s equal to target.
 *  
 * Example 1:
 *
 * Input: target = "10111"
 * Output: 3
 * Explanation: Initially, s = "00000".
 * Choose index i = 2: "00<u>000</u>" -> "00<u>111</u>"
 * Choose index i = 0: "<u>00111</u>" -> "<u>11000</u>"
 * Choose index i = 1: "1<u>1000</u>" -> "1<u>0111</u>"
 * We need at least 3 flip operations to form target.
 *
 * Example 2:
 *
 * Input: target = "101"
 * Output: 3
 * Explanation: Initially, s = "000".
 * Choose index i = 0: "<u>000</u>" -> "<u>111</u>"
 * Choose index i = 1: "1<u>11</u>" -> "1<u>00</u>"
 * Choose index i = 2: "10<u>0</u>" -> "10<u>1</u>"
 * We need at least 3 flip operations to form target.
 *
 * Example 3:
 *
 * Input: target = "00000"
 * Output: 0
 * Explanation: We do not need any operations since the initial s already equals target.
 *
 *  
 * Constraints:
 *
 * 	n == target.length
 * 	1 <= n <= 10^5
 * 	target[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-suffix-flips/
// discuss: https://leetcode.com/problems/minimum-suffix-flips/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let (mut flips, mut bit) = (0, '0');

        for digit in target.chars() {
            if digit != bit {
                flips += 1;
                bit = if bit == '0' { '1' } else { '0' };
            }
        }

        flips
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1529_example_1() {
        let target = "10111".to_string();

        let result = 3;

        assert_eq!(Solution::min_flips(target), result);
    }

    #[test]
    fn test_1529_example_2() {
        let target = "101".to_string();

        let result = 3;

        assert_eq!(Solution::min_flips(target), result);
    }

    #[test]
    fn test_1529_example_3() {
        let target = "00000".to_string();

        let result = 0;

        assert_eq!(Solution::min_flips(target), result);
    }
}
