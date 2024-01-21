/**
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 *
 * There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
 * Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
 * This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
 * Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
 * It's guaranteed that each city can reach city 0 after reorder.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/13/sample_1_1819.png" style="width: 311px; height: 189px;" />
 * Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
 * Output: 3
 * Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/13/sample_2_1819.png" style="width: 509px; height: 79px;" />
 * Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
 * Output: 2
 * Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
 *
 * Example 3:
 *
 * Input: n = 3, connections = [[1,0],[2,0]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 5 * 10^4
 * 	connections.length == n - 1
 * 	connections[i].length == 2
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
// discuss: https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let edges = {
            let mut result = vec![Vec::new(); connections.len() + 1];
            for connection in connections {
                result[connection[0] as usize].push((connection[1] as usize, true));
                result[connection[1] as usize].push((connection[0] as usize, false));
            }
            result
        };

        Self::dfs_helper(0, n as usize, &edges)
    }

    fn dfs_helper(node: usize, parent: usize, connections: &Vec<Vec<(usize, bool)>>) -> i32 {
        let mut result = 0;

        for &(neighbor, dir) in connections[node].iter() {
            if neighbor != parent {
                if dir {
                    result += 1
                }
                result += Self::dfs_helper(neighbor, node, connections);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1466_example_1() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];

        let result = 3;

        assert_eq!(Solution::min_reorder(n, connections), result);
    }

    #[test]
    fn test_1466_example_2() {
        let n = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];

        let result = 2;

        assert_eq!(Solution::min_reorder(n, connections), result);
    }

    #[test]
    fn test_1466_example_3() {
        let n = 3;
        let connections = vec![vec![1, 0], vec![2, 0]];

        let result = 0;

        assert_eq!(Solution::min_reorder(n, connections), result);
    }
}
