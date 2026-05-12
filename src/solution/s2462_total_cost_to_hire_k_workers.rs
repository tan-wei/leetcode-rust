/**
 * [2462] Total Cost to Hire K Workers
 *
 * You are given a 0-indexed integer array costs where costs[i] is the cost of hiring the i^th worker.
 * You are also given two integers k and candidates. We want to hire exactly k workers according to the following rules:
 *
 * 	You will run k sessions and hire exactly one worker in each session.
 * 	In each hiring session, choose the worker with the lowest cost from either the first candidates workers or the last candidates workers. Break the tie by the smallest index.
 *
 * 		For example, if costs = [3,2,7,7,1,2] and candidates = 2, then in the first hiring session, we will choose the 4^th worker because they have the lowest cost [<u>3,2</u>,7,7,<u>1,2</u>].
 * 		In the second hiring session, we will choose 1^st worker because they have the same lowest cost as 4^th worker but they have the smallest index [<u>3,2</u>,7,<u>7,2</u>]. Please note that the indexing may be changed in the process.
 *
 *
 * 	If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them. Break the tie by the smallest index.
 * 	A worker can only be chosen once.
 *
 * Return the total cost to hire exactly k workers.
 *  
 * Example 1:
 *
 * Input: costs = [17,12,10,2,7,2,11,20,8], k = 3, candidates = 4
 * Output: 11
 * Explanation: We hire 3 workers in total. The total cost is initially 0.
 * - In the first hiring round we choose the worker from [<u>17,12,10,2</u>,7,<u>2,11,20,8</u>]. The lowest cost is 2, and we break the tie by the smallest index, which is 3. The total cost = 0 + 2 = 2.
 * - In the second hiring round we choose the worker from [<u>17,12,10,7</u>,<u>2,11,20,8</u>]. The lowest cost is 2 (index 4). The total cost = 2 + 2 = 4.
 * - In the third hiring round we choose the worker from [<u>17,12,10,7,11,20,8</u>]. The lowest cost is 7 (index 3). The total cost = 4 + 7 = 11. Notice that the worker with index 3 was common in the first and last four workers.
 * The total hiring cost is 11.
 *
 * Example 2:
 *
 * Input: costs = [1,2,4,1], k = 3, candidates = 3
 * Output: 4
 * Explanation: We hire 3 workers in total. The total cost is initially 0.
 * - In the first hiring round we choose the worker from [<u>1,2,4,1</u>]. The lowest cost is 1, and we break the tie by the smallest index, which is 0. The total cost = 0 + 1 = 1. Notice that workers with index 1 and 2 are common in the first and last 3 workers.
 * - In the second hiring round we choose the worker from [<u>2,4,1</u>]. The lowest cost is 1 (index 2). The total cost = 1 + 1 = 2.
 * - In the third hiring round there are less than three candidates. We choose the worker from the remaining workers [<u>2,4</u>]. The lowest cost is 2 (index 0). The total cost = 2 + 2 = 4.
 * The total hiring cost is 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= costs.length <= 10^5
 * 	1 <= costs[i] <= 10^5
 * 	1 <= k, candidates <= costs.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/total-cost-to-hire-k-workers/
// discuss: https://leetcode.com/problems/total-cost-to-hire-k-workers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2462_example_1() {
        let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
        let k = 3;
        let candidates = 4;

        let result = 11;

        assert_eq!(Solution::total_cost(costs, k, candidates), result);
    }

    #[test]
    #[ignore]
    fn test_2462_example_2() {
        let costs = vec![1, 2, 4, 1];
        let k = 3;
        let candidates = 3;

        let result = 4;

        assert_eq!(Solution::total_cost(costs, k, candidates), result);
    }
}
