/**
 * [1192] Critical Connections in a Network
 *
 * There are n servers numbered from 0 to n - 1 connected by undirected server-to-server connections forming a network where connections[i] = [ai, bi] represents a connection between servers ai and bi. Any server can reach other servers directly or indirectly through the network.
 * A critical connection is a connection that, if removed, will make some servers unable to reach some other server.
 * Return all critical connections in the network in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/03/1537_ex1_2.png" style="width: 198px; height: 248px;" />
 * Input: n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
 * Output: [[1,3]]
 * Explanation: [[3,1]] is also accepted.
 *
 * Example 2:
 *
 * Input: n = 2, connections = [[0,1]]
 * Output: [[0,1]]
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^5
 * 	n - 1 <= connections.length <= 10^5
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 * 	There are no repeated connections.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/critical-connections-in-a-network/
// discuss: https://leetcode.com/problems/critical-connections-in-a-network/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/critical-connections-in-a-network/solutions/3355017/rust-implementation/
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

        for connection in connections {
            let (u, v) = (connection[0] as i32, connection[1] as i32);
            adj[u as usize].push(v);
            adj[v as usize].push(u);
        }

        let mut tim_in: Vec<i32> = vec![0; n as usize];
        let mut low: Vec<i32> = vec![0; n as usize];
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut bridges: Vec<Vec<i32>> = Vec::new();
        let mut timer: i32 = 0;

        Self::dfs_helper(
            &adj,
            -1,
            0,
            &mut visited,
            &mut timer,
            &mut tim_in,
            &mut low,
            &mut bridges,
        );

        bridges
    }

    fn dfs_helper(
        adj: &Vec<Vec<i32>>,
        parent: i32,
        node: i32,
        visited: &mut Vec<bool>,
        timer: &mut i32,
        time_in: &mut Vec<i32>,
        low: &mut Vec<i32>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        visited[node as usize] = true;
        time_in[node as usize] = *timer;
        low[node as usize] = *timer;

        *timer += 1;

        for &neighbor in &adj[node as usize] {
            if parent == neighbor {
                continue;
            }
            let neighbor_index = neighbor as usize;

            if !visited[neighbor_index] {
                Self::dfs_helper(adj, node, neighbor, visited, timer, time_in, low, bridges);

                low[node as usize] = low[neighbor_index].min(low[node as usize]);
                if low[neighbor_index] > time_in[node as usize] {
                    bridges.push(vec![node, neighbor]);
                }
            } else {
                low[node as usize] = low[neighbor_index].min(low[node as usize]);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1192_example_1() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        let result = vec![vec![3, 1]];

        assert_eq_sorted!(Solution::critical_connections(n, connections), result);
    }

    #[test]
    #[ignore]
    fn test_1192_example_2() {
        let n = 2;
        let connections = vec![vec![0, 1]];
        let result = vec![vec![0, 1]];

        assert_eq_sorted!(Solution::critical_connections(n, connections), result);
    }
}
