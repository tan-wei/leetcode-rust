/**
 * [0629] K Inverse Pairs Array
 *
 * For an integer array nums, an inverse pair is a pair of integers [i, j] where 0 <= i < j < nums.length and nums[i] > nums[j].
 * Given two integers n and k, return the number of different arrays consist of numbers from 1 to n such that there are exactly k inverse pairs. Since the answer can be huge, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 3, k = 0
 * Output: 1
 * Explanation: Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
 *
 * Example 2:
 *
 * Input: n = 3, k = 1
 * Output: 2
 * Explanation: The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	0 <= k <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-inverse-pairs-array/
// discuss: https://leetcode.com/problems/k-inverse-pairs-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        // There is at most n * (n - 1) / 2 pairs
        if k > n * (n - 1) / 2 {
            return 0;
        }

        // dp[k] represents the number of k different arrays consist of numbers
        let mut dp = vec![0; k as usize + 1];

        // Only one array meets this: [1, 2, 3, ..., n]
        dp[0] = 1;

        for i in 1..=n as usize {
            let mut row = vec![0; k as usize + 1];
            let mut sum = 0i32;
            for j in 0..=k as usize {
                sum += dp[j];
                if j >= i {
                    sum -= dp[j - i];
                }
                sum = sum.rem_euclid(MOD);
                row[j] = sum;
            }
            dp = row;
        }
        dp[k as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0629_example_1() {
        let n = 3;
        let k = 0;
        let result = 1;

        assert_eq!(Solution::k_inverse_pairs(n, k), result);
    }

    #[test]
    fn test_0629_example_2() {
        let n = 3;
        let k = 1;
        let result = 2;

        assert_eq!(Solution::k_inverse_pairs(n, k), result);
    }
}
