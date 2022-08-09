/**
 * [0785] Is Graph Bipartite?
 *
 * There is an undirected graph with n nodes, where each node is numbered between 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes that node u is adjacent to. More formally, for each v in graph[u], there is an undirected edge between node u and node v. The graph has the following properties:
 *
 * 	There are no self-edges (graph[u] does not contain u).
 * 	There are no parallel edges (graph[u] does not contain duplicate values).
 * 	If v is in graph[u], then u is in graph[v] (the graph is undirected).
 * 	The graph may not be connected, meaning there may be two nodes u and v such that there is no path between them.
 *
 * A graph is bipartite if the nodes can be partitioned into two independent sets A and B such that every edge in the graph connects a node in set A and a node in set B.
 * Return true if and only if it is bipartite.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/21/bi2.jpg" style="width: 222px; height: 222px;" />
 * Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
 * Output: false
 * Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/21/bi1.jpg" style="width: 222px; height: 222px;" />
 * Input: graph = [[1,3],[0,2],[1,3],[0,2]]
 * Output: true
 * Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
 *  
 * Constraints:
 *
 * 	graph.length == n
 * 	1 <= n <= 100
 * 	0 <= graph[u].length < n
 * 	0 <= graph[u][i] <= n - 1
 * 	graph[u] does not contain u.
 * 	All the values of graph[u] are unique.
 * 	If graph[u] contains v, then graph[v] contains u.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/is-graph-bipartite/
// discuss: https://leetcode.com/problems/is-graph-bipartite/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut colors: Vec<i32> = vec![-1; graph.len()];

        (0..graph.len()).all(|i| {
            if colors[i] == -1 {
                Self::dfs_helper(&graph, &mut colors, i, 1)
            } else {
                true
            }
        })
    }

    pub fn dfs_helper(graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>, i: usize, color: i32) -> bool {
        colors[i] = color;

        let mut result = true;
        for &j in graph[i].iter() {
            let j = j as usize;
            if colors[j] != -1 {
                if colors[j] == color {
                    return false;
                }
            } else {
                result &= Self::dfs_helper(graph, colors, j, !color);
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
    fn test_0785_example_1() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        let result = false;

        assert_eq!(Solution::is_bipartite(graph), result);
    }

    #[test]
    fn test_0785_example_2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        let result = true;

        assert_eq!(Solution::is_bipartite(graph), result);
    }
}
