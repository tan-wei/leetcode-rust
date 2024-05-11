/**
 * [1595] Minimum Cost to Connect Two Groups of Points
 *
 * You are given two groups of points where the first group has size1 points, the second group has size2 points, and size1 >= size2.
 * The cost of the connection between any two points are given in an size1 x size2 matrix where cost[i][j] is the cost of connecting point i of the first group and point j of the second group. The groups are connected if each point in both groups is connected to one or more points in the opposite group. In other words, each point in the first group must be connected to at least one point in the second group, and each point in the second group must be connected to at least one point in the first group.
 * Return the minimum cost it takes to connect the two groups.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex1.jpg" style="width: 322px; height: 243px;" />
 * Input: cost = [[15, 96], [36, 2]]
 * Output: 17
 * Explanation: The optimal way of connecting the groups is:
 * 1--A
 * 2--B
 * This results in a total cost of 17.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex2.jpg" style="width: 322px; height: 403px;" />
 * Input: cost = [[1, 3, 5], [4, 1, 1], [1, 5, 3]]
 * Output: 4
 * Explanation: The optimal way of connecting the groups is:
 * 1--A
 * 2--B
 * 2--C
 * 3--A
 * This results in a total cost of 4.
 * Note that there are multiple points connected to point 2 in the first group and point A in the second group. This does not matter as there is no limit to the number of points that can be connected. We only care about the minimum total cost.
 *
 * Example 3:
 *
 * Input: cost = [[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]]
 * Output: 10
 *
 *  
 * Constraints:
 *
 * 	size1 == cost.length
 * 	size2 == cost[i].length
 * 	1 <= size1, size2 <= 12
 * 	size1 >= size2
 * 	0 <= cost[i][j] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/
// discuss: https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/solutions/3175220/just-a-runnable-solution/
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let cost: Vec<Vec<_>> = cost
            .iter()
            .map(|v| v.iter().map(|&x| x as i64).collect())
            .collect();

        let (sz1, sz2) = (cost.len(), cost[0].len());
        let mut dp = vec![vec![1_000_000_007; 1 << sz2]; sz1 + 1];

        dp[sz1][(1 << sz2) - 1] = 0;

        for i in (0..sz1).rev() {
            for mask in (0..(1 << sz2)).rev() {
                for j in 0..sz2 {
                    dp[i][mask] = dp[i][mask].min(cost[i][j] + dp[i + 1][mask | (1 << j)]);
                    if mask & (1 << j) == 0 {
                        dp[i][mask] = dp[i][mask].min(cost[i][j] + dp[i][mask | (1 << j)]);
                    }
                }
            }
        }

        dp[0][0] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1595_example_1() {
        let cost = vec![vec![15, 96], vec![36, 2]];

        let result = 17;

        assert_eq!(Solution::connect_two_groups(cost), result);
    }

    #[test]
    fn test_1595_example_2() {
        let cost = vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]];

        let result = 4;

        assert_eq!(Solution::connect_two_groups(cost), result);
    }

    #[test]
    fn test_1595_example_3() {
        let cost = vec![
            vec![2, 5, 1],
            vec![3, 4, 7],
            vec![8, 1, 2],
            vec![6, 2, 4],
            vec![3, 8, 8],
        ];

        let result = 10;

        assert_eq!(Solution::connect_two_groups(cost), result);
    }
}
