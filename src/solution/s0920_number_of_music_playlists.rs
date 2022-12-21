/**
 * [0920] Number of Music Playlists
 *
 * Your music player contains n different songs. You want to listen to goal songs (not necessarily different) during your trip. To avoid boredom, you will create a playlist so that:
 *
 * 	Every song is played at least once.
 * 	A song can only be played again only if k other songs have been played.
 *
 * Given n, goal, and k, return the number of possible playlists that you can create. Since the answer can be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 3, goal = 3, k = 1
 * Output: 6
 * Explanation: There are 6 possible playlists: [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
 *
 * Example 2:
 *
 * Input: n = 2, goal = 3, k = 0
 * Output: 6
 * Explanation: There are 6 possible playlists: [1, 1, 2], [1, 2, 1], [2, 1, 1], [2, 2, 1], [2, 1, 2], and [1, 2, 2].
 *
 * Example 3:
 *
 * Input: n = 2, goal = 3, k = 1
 * Output: 2
 * Explanation: There are 2 possible playlists: [1, 2, 1] and [2, 1, 2].
 *
 *  
 * Constraints:
 *
 * 	0 <= k < n <= goal <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-music-playlists/
// discuss: https://leetcode.com/problems/number-of-music-playlists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i128 = 1_000_000_007;

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let mut dp = vec![vec![0 as i128; n as usize + 1]; goal as usize + 1];

        dp[0][0] = 1;

        for i in 1..(goal as usize + 1) {
            for j in 1..(n as usize + 1) {
                dp[i][j] += dp[i - 1][j - 1] * (n as i128 - j as i128 + 1);
                dp[i][j] += dp[i - 1][j] * std::cmp::max(j as i128 - k as i128, 0) as i128 % MOD;
                dp[i][j] %= MOD;
            }
        }

        dp[goal as usize][n as usize] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0920_example_1() {
        let n = 3;
        let goal = 3;
        let k = 1;
        let result = 6;

        assert_eq!(Solution::num_music_playlists(n, goal, k), result);
    }

    #[test]
    fn test_0920_example_2() {
        let n = 2;
        let goal = 3;
        let k = 0;
        let result = 6;

        assert_eq!(Solution::num_music_playlists(n, goal, k), result);
    }

    #[test]
    fn test_0920_example_3() {
        let n = 2;
        let goal = 3;
        let k = 1;
        let result = 2;

        assert_eq!(Solution::num_music_playlists(n, goal, k), result);
    }
}
