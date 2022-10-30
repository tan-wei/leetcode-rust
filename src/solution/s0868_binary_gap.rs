/**
 * [0868] Binary Gap
 *
 * Given a positive integer n, find and return the longest distance between any two adjacent 1's in the binary representation of n. If there are no two adjacent 1's, return 0.
 * Two 1's are adjacent if there are only 0's separating them (possibly no 0's). The distance between two 1's is the absolute difference between their bit positions. For example, the two 1's in "1001" have a distance of 3.
 *  
 * Example 1:
 *
 * Input: n = 22
 * Output: 2
 * Explanation: 22 in binary is "10110".
 * The first adjacent pair of 1's is "<u>1</u>0<u>1</u>10" with a distance of 2.
 * The second adjacent pair of 1's is "10<u>11</u>0" with a distance of 1.
 * The answer is the largest of these two distances, which is 2.
 * Note that "<u>1</u>01<u>1</u>0" is not a valid pair since there is a 1 separating the two 1's underlined.
 *
 * Example 2:
 *
 * Input: n = 8
 * Output: 0
 * Explanation: 8 in binary is "1000".
 * There are not any adjacent pairs of 1's in the binary representation of 8, so we return 0.
 *
 * Example 3:
 *
 * Input: n = 5
 * Output: 2
 * Explanation: 5 in binary is "101".
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-gap/
// discuss: https://leetcode.com/problems/binary-gap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let (mut max_dist, mut cur_dist) = (0, 0);
        let mut n = n >> n.trailing_zeros();

        while n > 0 {
            if n & 1 == 1 && cur_dist > 0 {
                max_dist = max_dist.max(cur_dist);
                cur_dist = 0;
            }

            cur_dist += 1;
            n >>= 1;
        }

        max_dist
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0868_example_1() {
        let n = 22;
        let result = 2;

        assert_eq!(Solution::binary_gap(n), result);
    }

    #[test]
    fn test_0868_example_2() {
        let n = 8;
        let result = 0;

        assert_eq!(Solution::binary_gap(n), result);
    }

    #[test]
    fn test_0868_example_3() {
        let n = 5;
        let result = 2;

        assert_eq!(Solution::binary_gap(n), result);
    }
}
