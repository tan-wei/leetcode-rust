/**
 * [1615] Maximal Network Rank
 *
 * There is an infrastructure of n cities with some number of roads connecting these cities. Each roads[i] = [ai, bi] indicates that there is a bidirectional road between cities ai and bi.
 * The network rank of two different cities is defined as the total number of directly connected roads to either city. If a road is directly connected to both cities, it is only counted once.
 * The maximal network rank of the infrastructure is the maximum network rank of all pairs of different cities.
 * Given the integer n and the array roads, return the maximal network rank of the entire infrastructure.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/ex1.png" style="width: 292px; height: 172px;" />
 *
 * Input: n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
 * Output: 4
 * Explanation: The network rank of cities 0 and 1 is 4 as there are 4 roads that are connected to either 0 or 1. The road between 0 and 1 is only counted once.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/ex2.png" style="width: 292px; height: 172px;" />
 *
 * Input: n = 5, roads = [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]
 * Output: 5
 * Explanation: There are 5 roads that are connected to cities 1 or 2.
 *
 * Example 3:
 *
 * Input: n = 8, roads = [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]]
 * Output: 5
 * Explanation: The network rank of 2 and 5 is 5. Notice that all the cities do not have to be connected.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 100
 * 	0 <= roads.length <= n * (n - 1) / 2
 * 	roads[i].length == 2
 * 	0 <= ai, bi <= n-1
 * 	ai != bi
 * 	Each pair of cities has at most one road connecting them.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-network-rank/
// discuss: https://leetcode.com/problems/maximal-network-rank/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut degree = vec![std::collections::HashSet::new(); n];
        for v in roads {
            degree[v[0] as usize].insert(v[1] as usize);
            degree[v[1] as usize].insert(v[0] as usize);
        }

        let mut result = 0;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let mut tmp = degree[i].len() + degree[j].len();
                if degree[i].contains(&j) {
                    tmp -= 1;
                }
                result = result.max(tmp);
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1615_example_1() {
        let n = 4;
        let roads = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]];

        let result = 4;

        assert_eq!(Solution::maximal_network_rank(n, roads), result);
    }

    #[test]
    fn test_1615_example_2() {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 4],
        ];

        let result = 5;

        assert_eq!(Solution::maximal_network_rank(n, roads), result);
    }

    fn test_1615_example_3() {
        let n = 8;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![2, 4],
            vec![5, 6],
            vec![5, 7],
        ];

        let result = 5;

        assert_eq!(Solution::maximal_network_rank(n, roads), result);
    }
}
