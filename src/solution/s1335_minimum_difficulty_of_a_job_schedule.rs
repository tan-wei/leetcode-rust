/**
 * [1335] Minimum Difficulty of a Job Schedule
 *
 * You want to schedule a list of jobs in d days. Jobs are dependent (i.e To work on the i^th job, you have to finish all the jobs j where 0 <= j < i).
 * You have to finish at least one task every day. The difficulty of a job schedule is the sum of difficulties of each day of the d days. The difficulty of a day is the maximum difficulty of a job done on that day.
 * You are given an integer array jobDifficulty and an integer d. The difficulty of the i^th job is jobDifficulty[i].
 * Return the minimum difficulty of a job schedule. If you cannot find a schedule for the jobs return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/untitled.png" style="width: 365px; height: 370px;" />
 * Input: jobDifficulty = [6,5,4,3,2,1], d = 2
 * Output: 7
 * Explanation: First day you can finish the first 5 jobs, total difficulty = 6.
 * Second day you can finish the last job, total difficulty = 1.
 * The difficulty of the schedule = 6 + 1 = 7
 *
 * Example 2:
 *
 * Input: jobDifficulty = [9,9,9], d = 4
 * Output: -1
 * Explanation: If you finish a job per day you will still have a free day. you cannot find a schedule for the given jobs.
 *
 * Example 3:
 *
 * Input: jobDifficulty = [1,1,1], d = 3
 * Output: 3
 * Explanation: The schedule is one job per day. total difficulty will be 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= jobDifficulty.length <= 300
 * 	0 <= jobDifficulty[i] <= 1000
 * 	1 <= d <= 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
// discuss: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2708959/rust-dp-solution/
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);
        if n < d as usize {
            return -1;
        }
        let mut dp = vec![vec![i32::MAX; d + 1]; n + 1];
        dp[0][0] = 0;
        for i in 1..=n {
            let mut max = 0;
            for j in (0..i).rev() {
                max = max.max(job_difficulty[j]);
                for k in 0..d {
                    if dp[j][k] < i32::MAX {
                        dp[i][k + 1] = dp[i][k + 1].min(dp[j][k] + max);
                    }
                }
            }
        }
        dp[n][d]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1335_example_1() {
        let job_difficulty = vec![6, 5, 4, 3, 2, 1];
        let d = 2;

        let result = 7;

        assert_eq!(Solution::min_difficulty(job_difficulty, d), result);
    }

    #[test]
    fn test_1335_example_2() {
        let job_difficulty = vec![9, 9, 9];
        let d = 4;

        let result = -1;

        assert_eq!(Solution::min_difficulty(job_difficulty, d), result);
    }

    #[test]
    fn test_1335_example_3() {
        let job_difficulty = vec![1, 1, 1];
        let d = 3;

        let result = 3;

        assert_eq!(Solution::min_difficulty(job_difficulty, d), result);
    }
}
