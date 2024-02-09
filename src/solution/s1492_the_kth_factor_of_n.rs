/**
 * [1492] The kth Factor of n
 *
 * You are given two positive integers n and k. A factor of an integer n is defined as an integer i where n % i == 0.
 * Consider a list of all factors of n sorted in ascending order, return the k^th factor in this list or return -1 if n has less than k factors.
 *  
 * Example 1:
 *
 * Input: n = 12, k = 3
 * Output: 3
 * Explanation: Factors list is [1, 2, 3, 4, 6, 12], the 3^rd factor is 3.
 *
 * Example 2:
 *
 * Input: n = 7, k = 2
 * Output: 7
 * Explanation: Factors list is [1, 7], the 2^nd factor is 7.
 *
 * Example 3:
 *
 * Input: n = 4, k = 4
 * Output: -1
 * Explanation: Factors list is [1, 2, 4], there is only 3 factors. We should return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= n <= 1000
 *
 *  
 * Follow up:
 * Could you solve this problem in less than O(n) complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-kth-factor-of-n/
// discuss: https://leetcode.com/problems/the-kth-factor-of-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        (1..=n)
            .filter(|i| n % i == 0)
            .skip((k - 1) as usize)
            .next()
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1492_example_1() {
        let n = 12;
        let k = 3;

        let result = 3;

        assert_eq!(Solution::kth_factor(n, k), result);
    }

    #[test]
    fn test_1492_example_2() {
        let n = 7;
        let k = 2;

        let result = 7;

        assert_eq!(Solution::kth_factor(n, k), result);
    }

    #[test]
    fn test_1492_example_3() {
        let n = 4;
        let k = 4;

        let result = -1;

        assert_eq!(Solution::kth_factor(n, k), result);
    }
}
