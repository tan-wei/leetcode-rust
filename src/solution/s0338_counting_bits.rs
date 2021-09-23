/**
 * [338] Counting Bits
 *
 * Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: [0,1,1]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: [0,1,1,2,1,2]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 * 3 --> 11
 * 4 --> 100
 * 5 --> 101
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^5
 *
 *  
 * Follow up:
 *
 * 	It is very easy to come up with a solution with a runtime of O(n log n). Can you do it in linear time O(n) and possibly in a single pass?
 * 	Can you do it without using any built-in function (i.e., like __builtin_popcount in C++)?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/counting-bits/
// discuss: https://leetcode.com/problems/counting-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (1..=(n as usize)).fold(vec![0; n as usize + 1], |mut acc, x| {
            acc[x] = acc[x & (x - 1)] + 1;
            acc
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0338_example_1() {
        let n = 2;
        let result = vec![0, 1, 1];

        assert_eq!(Solution::count_bits(n), result);
    }

    #[test]
    fn test_0338_example_2() {
        let n = 5;
        let result = vec![0, 1, 1, 2, 1, 2];

        assert_eq!(Solution::count_bits(n), result);
    }
}
