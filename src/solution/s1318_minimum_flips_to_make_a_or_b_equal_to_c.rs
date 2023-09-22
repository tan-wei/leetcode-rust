/**
 * [1318] Minimum Flips to Make a OR b Equal to c
 *
 * Given 3 positives numbers a, b and c. Return the minimum flips required in some bits of a and b to make ( a OR b == c ). (bitwise OR operation).<br />
 * Flip operation consists of change any single bit 1 to 0 or change the bit 0 to 1 in their binary representation.
 *
 *
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/06/sample_3_1676.png" style="width: 260px; height: 87px;" />
 *
 *
 * Input: a = 2, b = 6, c = 5
 * Output: 3
 * Explanation: After flips a = 1 , b = 4 , c = 5 such that (a OR b == c)
 *
 * Example 2:
 *
 *
 * Input: a = 4, b = 2, c = 7
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: a = 1, b = 2, c = 3
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= a <= 10^9
 * 	1 <= b <= 10^9
 * 	1 <= c <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
// discuss: https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        (((a | b) ^ c).count_ones() + ((a & b) & !c).count_ones()) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1318_example_1() {
        let a = 2;
        let b = 6;
        let c = 5;

        let result = 3;

        assert_eq!(Solution::min_flips(a, b, c), result);
    }

    #[test]
    fn test_1318_example_2() {
        let a = 4;
        let b = 2;
        let c = 7;

        let result = 1;

        assert_eq!(Solution::min_flips(a, b, c), result);
    }

    #[test]
    fn test_1318_example_3() {
        let a = 1;
        let b = 2;
        let c = 3;

        let result = 0;

        assert_eq!(Solution::min_flips(a, b, c), result);
    }
}
