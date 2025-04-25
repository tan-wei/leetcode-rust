/**
 * [1976] Number of Ways to Arrive at Destination
 *
 * You are in a city that consists of n intersections numbered from 0 to n - 1 with bi-directional roads between some intersections. The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.
 * You are given an integer n and a 2D integer array roads where roads[i] = [ui, vi, timei] means that there is a road between intersections ui and vi that takes timei minutes to travel. You want to know in how many ways you can travel from intersection 0 to intersection n - 1 in the shortest amount of time.
 * Return the number of ways you can arrive at your destination in the shortest amount of time. Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2025/02/14/1976_corrected.png" style="width: 255px; height: 400px;" />
 * Input: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
 * Output: 4
 * Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
 * The four ways to get there in 7 minutes are:
 * - 0 ➝ 6
 * - 0 ➝ 4 ➝ 6
 * - 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
 * - 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
 *
 * Example 2:
 *
 * Input: n = 2, roads = [[1,0,10]]
 * Output: 1
 * Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 200
 * 	n - 1 <= roads.length <= n * (n - 1) / 2
 * 	roads[i].length == 3
 * 	0 <= ui, vi <= n - 1
 * 	1 <= timei <= 10^9
 * 	ui != vi
 * 	There is at most one road connecting any two intersections.
 * 	You can reach any intersection from any other intersection.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
// discuss: https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1976_example_1() {
        let n = 7;
        let roads = vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ];

        let result = 4;

        assert_eq!(Solution::count_paths(n, roads), result);
    }

    #[test]
    #[ignore]
    fn test_1976_example_2() {
        let n = 2;
        let roads = vec![vec![1, 0, 10]];

        let result = 1;

        assert_eq!(Solution::count_paths(n, roads), result);
    }
}
