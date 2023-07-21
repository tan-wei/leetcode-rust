/**
 * [1223] Dice Roll Simulation
 *
 * A die simulator generates a random number from 1 to 6 for each roll. You introduced a constraint to the generator such that it cannot roll the number i more than rollMax[i] (1-indexed) consecutive times.
 * Given an array of integers rollMax and an integer n, return the number of distinct sequences that can be obtained with exact n rolls. Since the answer may be too large, return it modulo 10^9 + 7.
 * Two sequences are considered different if at least one element differs from each other.
 *  
 * Example 1:
 *
 * Input: n = 2, rollMax = [1,1,2,2,2,3]
 * Output: 34
 * Explanation: There will be 2 rolls of die, if there are no constraints on the die, there are 6 * 6 = 36 possible combinations. In this case, looking at rollMax array, the numbers 1 and 2 appear at most once consecutively, therefore sequences (1,1) and (2,2) cannot occur, so the final answer is 36-2 = 34.
 *
 * Example 2:
 *
 * Input: n = 2, rollMax = [1,1,1,1,1,1]
 * Output: 30
 *
 * Example 3:
 *
 * Input: n = 3, rollMax = [1,1,1,2,2,3]
 * Output: 181
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 5000
 * 	rollMax.length == 6
 * 	1 <= rollMax[i] <= 15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/dice-roll-simulation/
// discuss: https://leetcode.com/problems/dice-roll-simulation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: i64 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/dice-roll-simulation/solutions/2779607/rust-dp-solution/
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![[1i64; 6]; n];

        for i in 1..n {
            for j in 0..6 {
                dp[i][j] = 0;
                for k in 0..6 {
                    dp[i][j] += dp[i - 1][k];
                }

                if roll_max[j] as usize <= i {
                    if roll_max[j] as usize == i {
                        dp[i][j] -= 1;
                    } else {
                        for k in 0..6 {
                            if k != j {
                                dp[i][j] -= dp[i - 1 - roll_max[j] as usize][k];
                            }
                        }
                    }
                }
                dp[i][j] %= MOD;
                if dp[i][j] < 0 {
                    dp[i][j] += MOD;
                }
            }
        }

        (dp[n - 1].iter().sum::<i64>() % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1223_example_1() {
        let n = 2;
        let roll_max = vec![1, 1, 2, 2, 2, 3];
        let result = 34;

        assert_eq!(Solution::die_simulator(n, roll_max), result);
    }

    #[test]
    fn test_1223_example_2() {
        let n = 2;
        let roll_max = vec![1, 1, 1, 1, 1, 1];
        let result = 30;

        assert_eq!(Solution::die_simulator(n, roll_max), result);
    }

    #[test]
    fn test_1223_example_3() {
        let n = 3;
        let roll_max = vec![1, 1, 1, 2, 2, 3];
        let result = 181;

        assert_eq!(Solution::die_simulator(n, roll_max), result);
    }
}
