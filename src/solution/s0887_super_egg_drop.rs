/**
 * [0887] Super Egg Drop
 *
 * You are given k identical eggs and you have access to a building with n floors labeled from 1 to n.
 * You know that there exists a floor f where 0 <= f <= n such that any egg dropped at a floor higher than f will break, and any egg dropped at or below floor f will not break.
 * Each move, you may take an unbroken egg and drop it from any floor x (where 1 <= x <= n). If the egg breaks, you can no longer use it. However, if the egg does not break, you may reuse it in future moves.
 * Return the minimum number of moves that you need to determine with certainty what the value of f is.
 *  
 * Example 1:
 *
 * Input: k = 1, n = 2
 * Output: 2
 * Explanation:
 * Drop the egg from floor 1. If it breaks, we know that f = 0.
 * Otherwise, drop the egg from floor 2. If it breaks, we know that f = 1.
 * If it does not break, then we know f = 2.
 * Hence, we need at minimum 2 moves to determine with certainty what the value of f is.
 *
 * Example 2:
 *
 * Input: k = 2, n = 6
 * Output: 3
 *
 * Example 3:
 *
 * Input: k = 3, n = 14
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 100
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-egg-drop/
// discuss: https://leetcode.com/problems/super-egg-drop/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/super-egg-drop/solutions/631518/rust-dp-0ms-100/
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = Vec::with_capacity(n + 1);
        for i in 0..=n {
            dp.push(i);
        }
        let mut last_dp = dp.clone();

        for _ in 2..=k {
            dp.clear();
            dp.push(0);
            for i in 1..=n {
                if dp[i - 1] >= n {
                    break;
                }
                dp.push(1 + dp[i - 1] + last_dp[i - 1]);
            }
            last_dp[..dp.len()].copy_from_slice(&dp);
        }
        dp.len() as i32 - 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0887_example_1() {
        let k = 1;
        let n = 2;
        let result = 2;

        assert_eq!(Solution::super_egg_drop(k, n), result);
    }

    #[test]
    fn test_0887_example_2() {
        let k = 2;
        let n = 6;
        let result = 3;

        assert_eq!(Solution::super_egg_drop(k, n), result);
    }

    #[test]
    fn test_0887_example_3() {
        let k = 3;
        let n = 14;
        let result = 4;

        assert_eq!(Solution::super_egg_drop(k, n), result);
    }
}
