/**
 * [1601] Maximum Number of Achievable Transfer Requests
 *
 * We have n buildings numbered from 0 to n - 1. Each building has a number of employees. It's transfer season, and some employees want to change the building they reside in.
 * You are given an array requests where requests[i] = [fromi, toi] represents an employee's request to transfer from building fromi to building toi.
 * All buildings are full, so a list of requests is achievable only if for each building, the net change in employee transfers is zero. This means the number of employees leaving is equal to the number of employees moving in. For example if n = 3 and two employees are leaving building 0, one is leaving building 1, and one is leaving building 2, there should be two employees moving to building 0, one employee moving to building 1, and one employee moving to building 2.
 * Return the maximum number of achievable requests.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/move1.jpg" style="width: 600px; height: 406px;" />
 * Input: n = 5, requests = [[0,1],[1,0],[0,1],[1,2],[2,0],[3,4]]
 * Output: 5
 * Explantion: Let's see the requests:
 * From building 0 we have employees x and y and both want to move to building 1.
 * From building 1 we have employees a and b and they want to move to buildings 2 and 0 respectively.
 * From building 2 we have employee z and they want to move to building 0.
 * From building 3 we have employee c and they want to move to building 4.
 * From building 4 we don't have any requests.
 * We can achieve the requests of users x and b by swapping their places.
 * We can achieve the requests of users y, a and z by swapping the places in the 3 buildings.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/move2.jpg" style="width: 450px; height: 327px;" />
 * Input: n = 3, requests = [[0,0],[1,2],[2,1]]
 * Output: 3
 * Explantion: Let's see the requests:
 * From building 0 we have employee x and they want to stay in the same building 0.
 * From building 1 we have employee y and they want to move to building 2.
 * From building 2 we have employee z and they want to move to building 1.
 * We can achieve all the requests.
 * Example 3:
 *
 * Input: n = 4, requests = [[0,3],[3,1],[1,2],[2,0]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 * 	1 <= requests.length <= 16
 * 	requests[i].length == 2
 * 	0 <= fromi, toi < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/
// discuss: https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; n as usize];
        let i = 0;
        Self::dfs_helper(&mut dp, n as usize, &requests, i)
    }

    fn dfs_helper(dp: &mut Vec<i32>, _n: usize, requests: &Vec<Vec<i32>>, i: usize) -> i32 {
        if i == requests.len() {
            return if dp.iter().all(|&x| x == 0) {
                0
            } else {
                std::i32::MIN
            };
        }
        dp[requests[i][0] as usize] -= 1;
        dp[requests[i][1] as usize] += 1;
        let take = 1 + Self::dfs_helper(dp, _n, requests, i + 1);
        dp[requests[i][0] as usize] += 1;
        dp[requests[i][1] as usize] -= 1;
        take.max(Self::dfs_helper(dp, _n, requests, i + 1))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1601_example_1() {
        let n = 5;
        let requests = vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ];

        let result = 5;

        assert_eq!(Solution::maximum_requests(n, requests), result);
    }

    #[test]
    fn test_1601_example_2() {
        let n = 3;
        let requests = vec![vec![0, 0], vec![1, 2], vec![2, 1]];

        let result = 3;

        assert_eq!(Solution::maximum_requests(n, requests), result);
    }

    #[test]
    fn test_1601_example_3() {
        let n = 5;
        let requests = vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]];

        let result = 4;

        assert_eq!(Solution::maximum_requests(n, requests), result);
    }
}
