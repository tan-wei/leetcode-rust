/**
 * [1235] Maximum Profit in Job Scheduling
 *
 * We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
 * You're given the startTime, endTime and profit arrays, return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.
 * If you choose a job that ends at time X you will be able to start another job that starts at time X.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample1_1584.png" style="width: 380px; height: 154px;" />
 *
 * Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
 * Output: 120
 * Explanation: The subset chosen is the first and fourth job.
 * Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample22_1584.png" style="width: 600px; height: 112px;" />
 *
 * Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
 * Output: 150
 * Explanation: The subset chosen is the first, fourth and fifth job.
 * Profit obtained 150 = 20 + 70 + 60.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/10/sample3_1584.png" style="width: 400px; height: 112px;" />
 *
 * Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
 * Output: 6
 *
 *  
 * Constraints:
 *
 * 	1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
 * 	1 <= startTime[i] < endTime[i] <= 10^9
 * 	1 <= profit[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-profit-in-job-scheduling/
// discuss: https://leetcode.com/problems/maximum-profit-in-job-scheduling/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2851123/rust-15ms/
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<_> = start_time
            .into_iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((a, b), c)| (a, *b, *c))
            .collect();

        jobs.sort();

        let mut dp = vec![-1; jobs.len()];

        Self::dfs_helper(0, &jobs, &mut dp)
    }

    fn dfs_helper(i: usize, jobs: &Vec<(i32, i32, i32)>, dp: &mut Vec<i32>) -> i32 {
        if i >= jobs.len() {
            return 0;
        }
        if dp[i] >= 0 {
            return dp[i];
        }

        let k = jobs.partition_point(|&job| job.0 < jobs[i].1);
        dp[i] = Self::dfs_helper(i + 1, jobs, dp).max(jobs[i].2 + Self::dfs_helper(k, jobs, dp));
        dp[i]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1235_example_1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        let result = 120;

        assert_eq!(
            Solution::job_scheduling(start_time, end_time, profit),
            result
        );
    }

    #[test]
    fn test_1235_example_2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        let result = 150;

        assert_eq!(
            Solution::job_scheduling(start_time, end_time, profit),
            result
        );
    }

    #[test]
    fn test_1235_example_3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        let result = 6;

        assert_eq!(
            Solution::job_scheduling(start_time, end_time, profit),
            result
        );
    }
}
