/**
 * [1155] Number of Dice Rolls With Target Sum
 *
 * You have n dice, and each die has k faces numbered from 1 to k.
 * Given three integers n, k, and target, return the number of possible ways (out of the k^n total ways) to roll the dice, so the sum of the face-up numbers equals target. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 1, k = 6, target = 3
 * Output: 1
 * Explanation: You throw one die with 6 faces.
 * There is only one way to get a sum of 3.
 *
 * Example 2:
 *
 * Input: n = 2, k = 6, target = 7
 * Output: 6
 * Explanation: You throw two dice, each with 6 faces.
 * There are 6 ways to get a sum of 7: 1+6, 2+5, 3+4, 4+3, 5+2, 6+1.
 *
 * Example 3:
 *
 * Input: n = 30, k = 30, target = 500
 * Output: 222616187
 * Explanation: The answer must be returned modulo 10^9 + 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= n, k <= 30
 * 	1 <= target <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/
// discuss: https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MODULO: usize = 1000000007;

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/solutions/2610569/rust-bottom-up-dp/
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let (mut dp_prev, mut dp_curr) = (vec![0; target + 1], vec![0; target + 1]);
        dp_prev[0] = 1;
        for _ in 0..n {
            for j in 1..=target {
                for m in 1..=k {
                    if m <= j {
                        dp_curr[j] = (dp_curr[j] + dp_prev[j - m]) % MODULO;
                    }
                }
            }
            std::mem::swap(&mut dp_curr, &mut dp_prev);
            dp_curr.fill(0);
        }
        dp_prev[target] as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1155_example_1() {
        let n = 1;
        let k = 6;
        let target = 3;
        let result = 1;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), result);
    }

    #[test]
    fn test_1155_example_2() {
        let n = 2;
        let k = 6;
        let target = 7;
        let result = 6;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), result);
    }

    #[test]
    fn test_1155_example_3() {
        let n = 30;
        let k = 30;
        let target = 500;
        let result = 222616187;

        assert_eq!(Solution::num_rolls_to_target(n, k, target), result);
    }
}
