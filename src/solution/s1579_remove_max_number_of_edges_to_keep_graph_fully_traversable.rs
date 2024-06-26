/**
 * [1579] Remove Max Number of Edges to Keep Graph Fully Traversable
 *
 * Alice and Bob have an undirected graph of n nodes and three types of edges:
 *
 * 	Type 1: Can be traversed by Alice only.
 * 	Type 2: Can be traversed by Bob only.
 * 	Type 3: Can be traversed by both Alice and Bob.
 *
 * Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.
 * Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex1.png" style="width: 179px; height: 191px;" />
 *
 * Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
 * Output: 2
 * Explanation: If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex2.png" style="width: 178px; height: 190px;" />
 *
 * Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
 * Output: 0
 * Explanation: Notice that removing any edge will not make the graph fully traversable by Alice and Bob.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex3.png" style="width: 178px; height: 190px;" />
 *
 * Input: n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
 * Output: -1
 * Explanation: In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it's impossible to make the graph fully traversable.
 *  
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	1 <= edges.length <= min(10^5, 3 * n * (n - 1) / 2)
 * 	edges[i].length == 3
 * 	1 <= typei <= 3
 * 	1 <= ui < vi <= n
 * 	All tuples (typei, ui, vi) are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/
// discuss: https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let (n, tot) = (n as usize, edges.len() as i32);
        let mut alice_comps = (0..=n).collect::<Vec<usize>>();
        let mut bob_comps = (0..=n).collect::<Vec<usize>>();

        let mut alice_sz = n;
        let mut bob_sz = n;

        fn find(node: usize, parents: &mut Vec<usize>) -> usize {
            if parents[node] != node {
                let parent = find(parents[node], parents);
                parents[node] = parent;
            }
            parents[node]
        }

        fn union(node1: usize, node2: usize, parents: &mut Vec<usize>, sz: &mut usize) -> bool {
            let (p1, p2) = (find(node1, parents), find(node2, parents));
            if p1 != p2 {
                *sz -= 1;
                parents[p2] = p1;
                return true;
            }
            false
        }

        let (mut alice_edges, mut bob_edges) = (Vec::new(), Vec::new());
        let mut result = 0;

        for edge in edges {
            if edge[0] == 3 {
                let naz = union(
                    edge[1] as usize,
                    edge[2] as usize,
                    &mut alice_comps,
                    &mut alice_sz,
                );
                let nbz = union(
                    edge[1] as usize,
                    edge[2] as usize,
                    &mut bob_comps,
                    &mut bob_sz,
                );
                if naz || nbz {
                    result += 1;
                }
            } else if edge[0] == 2 {
                bob_edges.push(edge);
            } else {
                alice_edges.push(edge);
            }
        }

        for edge in alice_edges {
            if union(
                edge[1] as usize,
                edge[2] as usize,
                &mut alice_comps,
                &mut alice_sz,
            ) {
                result += 1;
            }
        }

        for edge in bob_edges {
            if union(
                edge[1] as usize,
                edge[2] as usize,
                &mut bob_comps,
                &mut bob_sz,
            ) {
                result += 1;
            }
        }

        if alice_sz != 1 || bob_sz != 1 {
            return -1;
        }

        tot - result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1579_example_1() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];

        let result = 2;

        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }

    #[test]
    fn test_1579_example_2() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];

        let result = 0;

        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }

    #[test]
    fn test_1579_example_3() {
        let n = 4;
        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];

        let result = -1;

        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }
}
