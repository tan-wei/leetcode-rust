/**
 * [0847] Shortest Path Visiting All Nodes
 *
 * You have an undirected, connected graph of n nodes labeled from 0 to n - 1. You are given an array graph where graph[i] is a list of all the nodes connected with node i by an edge.
 * Return the length of the shortest path that visits every node. You may start and stop at any node, you may revisit nodes multiple times, and you may reuse edges.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/12/shortest1-graph.jpg" style="width: 222px; height: 183px;" />
 * Input: graph = [[1,2,3],[0],[0],[0]]
 * Output: 4
 * Explanation: One possible path is [1,0,2,0,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/12/shortest2-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: graph = [[1],[0,2,4],[1,3,4],[2],[1,2]]
 * Output: 4
 * Explanation: One possible path is [0,1,4,2,3]
 *
 *  
 * Constraints:
 *
 * 	n == graph.length
 * 	1 <= n <= 12
 * 	0 <= graph[i].length < n
 * 	graph[i] does not contain i.
 * 	If graph[a] contains b, then graph[b] contains a.
 * 	The input graph is always connected.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-visiting-all-nodes/
// discuss: https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; graph.len()]; 1 << graph.len()];

        let mut vd = std::collections::VecDeque::new();
        for i in 0..graph.len() {
            vd.push_back((i, 1 << i));
            visited[1 << i][i] = true;
        }

        let end = (1 << graph.len()) - 1;
        for len in 0.. {
            for _ in 0..vd.len() {
                if let Some((i, mask)) = vd.pop_front() {
                    if mask == end {
                        return len;
                    }

                    for &j in &graph[i] {
                        if !visited[mask | (1 << j)][j as usize] {
                            visited[mask | (1 << j)][j as usize] = true;
                            vd.push_back((j as usize, mask | (1 << j)));
                        }
                    }
                }
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0847_example_1() {
        let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let result = 4;

        assert_eq!(Solution::shortest_path_length(graph), result);
    }

    #[test]
    fn test_0847_example_2() {
        let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
        let result = 4;

        assert_eq!(Solution::shortest_path_length(graph), result);
    }
}
