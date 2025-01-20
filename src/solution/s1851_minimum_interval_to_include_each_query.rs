/**
 * [1851] Minimum Interval to Include Each Query
 *
 * You are given a 2D integer array intervals, where intervals[i] = [lefti, righti] describes the i^th interval starting at lefti and ending at righti (inclusive). The size of an interval is defined as the number of integers it contains, or more formally righti - lefti + 1.
 * You are also given an integer array queries. The answer to the j^th query is the size of the smallest interval i such that lefti <= queries[j] <= righti. If no such interval exists, the answer is -1.
 * Return an array containing the answers to the queries.
 *  
 * Example 1:
 *
 * Input: intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]
 * Output: [3,3,1,4]
 * Explanation: The queries are processed as follows:
 * - Query = 2: The interval [2,4] is the smallest interval containing 2. The answer is 4 - 2 + 1 = 3.
 * - Query = 3: The interval [2,4] is the smallest interval containing 3. The answer is 4 - 2 + 1 = 3.
 * - Query = 4: The interval [4,4] is the smallest interval containing 4. The answer is 4 - 4 + 1 = 1.
 * - Query = 5: The interval [3,6] is the smallest interval containing 5. The answer is 6 - 3 + 1 = 4.
 *
 * Example 2:
 *
 * Input: intervals = [[2,3],[2,5],[1,8],[20,25]], queries = [2,19,5,22]
 * Output: [2,-1,4,6]
 * Explanation: The queries are processed as follows:
 * - Query = 2: The interval [2,3] is the smallest interval containing 2. The answer is 3 - 2 + 1 = 2.
 * - Query = 19: None of the intervals contain 19. The answer is -1.
 * - Query = 5: The interval [2,5] is the smallest interval containing 5. The answer is 5 - 2 + 1 = 4.
 * - Query = 22: The interval [20,25] is the smallest interval containing 22. The answer is 25 - 20 + 1 = 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^5
 * 	1 <= queries.length <= 10^5
 * 	intervals[i].length == 2
 * 	1 <= lefti <= righti <= 10^7
 * 	1 <= queries[j] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-interval-to-include-each-query/
// discuss: https://leetcode.com/problems/minimum-interval-to-include-each-query/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1851_example_1() {
        let intervals = vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]];
        let queries = vec![2, 3, 4, 5];

        let result = vec![3, 3, 1, 4];

        assert_eq!(Solution::min_interval(intervals, queries), result);
    }

    #[test]
    #[ignore]
    fn test_1851_example_2() {
        let intervals = vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]];
        let queries = vec![2, 19, 5, 22];

        let result = vec![2, -1, 4, 6];

        assert_eq!(Solution::min_interval(intervals, queries), result);
    }
}
