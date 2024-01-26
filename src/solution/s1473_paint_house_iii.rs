/**
 * [1473] Paint House III
 *
 * There is a row of m houses in a small city, each house must be painted with one of the n colors (labeled from 1 to n), some houses that have been painted last summer should not be painted again.
 * A neighborhood is a maximal group of continuous houses that are painted with the same color.
 *
 * 	For example: houses = [1,2,2,3,3,2,1,1] contains 5 neighborhoods [{1}, {2,2}, {3,3}, {2}, {1,1}].
 *
 * Given an array houses, an m x n matrix cost and an integer target where:
 *
 * 	houses[i]: is the color of the house i, and 0 if the house is not painted yet.
 * 	cost[i][j]: is the cost of paint the house i with the color j + 1.
 *
 * Return the minimum cost of painting all the remaining houses in such a way that there are exactly target neighborhoods. If it is not possible, return -1.
 *  
 * Example 1:
 *
 * Input: houses = [0,0,0,0,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
 * Output: 9
 * Explanation: Paint houses of this way [1,2,2,1,1]
 * This array contains target = 3 neighborhoods, [{1}, {2,2}, {1,1}].
 * Cost of paint all houses (1 + 1 + 1 + 1 + 5) = 9.
 *
 * Example 2:
 *
 * Input: houses = [0,2,1,2,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
 * Output: 11
 * Explanation: Some houses are already painted, Paint the houses of this way [2,2,1,2,2]
 * This array contains target = 3 neighborhoods, [{2,2}, {1}, {2,2}].
 * Cost of paint the first and last house (10 + 1) = 11.
 *
 * Example 3:
 *
 * Input: houses = [3,1,2,3], cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]], m = 4, n = 3, target = 3
 * Output: -1
 * Explanation: Houses are already painted with a total of 4 neighborhoods [{3},{1},{2},{3}] different of target = 3.
 *
 *  
 * Constraints:
 *
 * 	m == houses.length == cost.length
 * 	n == cost[i].length
 * 	1 <= m <= 100
 * 	1 <= n <= 20
 * 	1 <= target <= m
 * 	0 <= houses[i] <= n
 * 	1 <= cost[i][j] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/paint-house-iii/
// discuss: https://leetcode.com/problems/paint-house-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/paint-house-iii/solutions/3604808/rust-bottom-up-approach/
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        const MAX: i32 = 1_000_001;
        let mut dp = vec![vec![vec![MAX; target as usize + 1]; n as usize]; m as usize + 1];
        for j in 0..n as usize {
            if houses[0] as usize == j + 1 {
                dp[0][j][1] = 0;
            } else if houses[0] == 0 {
                dp[0][j][1] = cost[0][j];
            }
        }

        for i in 1..m as usize {
            for j in 0..n as usize {
                for k in 1..=target as usize {
                    if houses[i] != 0 && j as i32 + 1 != houses[i] {
                        continue;
                    }
                    let mut curr_cost = MAX;
                    for prev_color in 0..n as usize {
                        if j == prev_color {
                            curr_cost = curr_cost.min(dp[i - 1][prev_color][k]);
                        } else {
                            curr_cost = curr_cost.min(dp[i - 1][prev_color][k - 1]);
                        }
                    }

                    let cost_to_paint = if houses[i] != 0 { 0 } else { cost[i][j] };
                    dp[i][j][k] = curr_cost + cost_to_paint;
                }
            }
        }

        let mut result = MAX;
        for j in 0..n as usize {
            result = result.min(dp[m as usize - 1][j][target as usize]);
        }
        if result == MAX {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1473_example_1() {
        let houses = vec![0, 0, 0, 0, 0];
        let cost = vec![
            vec![1, 10],
            vec![10, 1],
            vec![10, 1],
            vec![1, 10],
            vec![5, 1],
        ];
        let m = 5;
        let n = 2;
        let target = 3;

        let result = 9;

        assert_eq!(Solution::min_cost(houses, cost, m, n, target), result);
    }

    #[test]
    fn test_1473_example_2() {
        let houses = vec![0, 2, 1, 2, 0];
        let cost = vec![
            vec![1, 10],
            vec![10, 1],
            vec![10, 1],
            vec![1, 10],
            vec![5, 1],
        ];
        let m = 5;
        let n = 2;
        let target = 3;

        let result = 11;

        assert_eq!(Solution::min_cost(houses, cost, m, n, target), result);
    }

    #[test]
    fn test_1473_example_3() {
        let houses = vec![3, 1, 2, 3];
        let cost = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let m = 4;
        let n = 3;
        let target = 3;

        let result = -1;

        assert_eq!(Solution::min_cost(houses, cost, m, n, target), result);
    }
}
