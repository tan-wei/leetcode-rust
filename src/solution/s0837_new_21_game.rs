/**
 * [0837] New 21 Game
 *
 * Alice plays the following game, loosely based on the card game "21".
 * Alice starts with 0 points and draws numbers while she has less than k points. During each draw, she gains an integer number of points randomly from the range [1, maxPts], where maxPts is an integer. Each draw is independent and the outcomes have equal probabilities.
 * Alice stops drawing numbers when she gets k or more points.
 * Return the probability that Alice has n or fewer points.
 * Answers within 10^-5 of the actual answer are considered accepted.
 *  
 * Example 1:
 *
 * Input: n = 10, k = 1, maxPts = 10
 * Output: 1.00000
 * Explanation: Alice gets a single card, then stops.
 *
 * Example 2:
 *
 * Input: n = 6, k = 1, maxPts = 10
 * Output: 0.60000
 * Explanation: Alice gets a single card, then stops.
 * In 6 out of 10 possibilities, she is at or below 6 points.
 *
 * Example 3:
 *
 * Input: n = 21, k = 17, maxPts = 10
 * Output: 0.73278
 *
 *  
 * Constraints:
 *
 * 	0 <= k <= n <= 10^4
 * 	1 <= maxPts <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/new-21-game/
// discuss: https://leetcode.com/problems/new-21-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0f64;
        }

        let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);

        let mut dp = vec![0.0; n + 1];
        let mut sum = 1.0;
        dp[0] = 1.0;

        for i in 1..=n {
            if k + max_pts > i {
                dp[i] += sum / max_pts as f64;
            }

            if i < k {
                sum += dp[i];
            }

            if i >= max_pts {
                sum -= dp[i - max_pts];
            }
        }

        dp[k..=n].iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0837_example_1() {
        let n = 10;
        let k = 1;
        let max_pts = 10;
        let result = 1.00000;

        assert_f64_near!(Solution::new21_game(n, k, max_pts), result);
    }

    #[test]
    fn test_0837_example_2() {
        let n = 6;
        let k = 1;
        let max_pts = 10;
        let result = 0.60000;

        assert_f64_near!(Solution::new21_game(n, k, max_pts), result);
    }

    #[test]
    fn test_0837_example_3() {
        let n = 21;
        let k = 17;
        let max_pts = 10;
        let result = 0.7327777870686077;

        assert_f64_near!(Solution::new21_game(n, k, max_pts), result);
    }
}
