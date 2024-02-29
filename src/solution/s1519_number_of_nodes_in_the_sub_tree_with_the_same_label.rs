/**
 * [1519] Number of Nodes in the Sub-Tree With the Same Label
 *
 * You are given a tree (i.e. a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. The root of the tree is the node 0, and each node of the tree has a label which is a lower-case character given in the string labels (i.e. The node with the number i has the label labels[i]).
 * The edges array is given on the form edges[i] = [ai, bi], which means there is an edge between nodes ai and bi in the tree.
 * Return an array of size n where ans[i] is the number of nodes in the subtree of the i^th node which have the same label as node i.
 * A subtree of a tree T is the tree consisting of a node in T and all of its descendant nodes.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/01/q3e1.jpg" style="width: 400px; height: 291px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], labels = "abaedcd"
 * Output: [2,1,1,1,1,1,1]
 * Explanation: Node 0 has label 'a' and its sub-tree has node 2 with label 'a' as well, thus the answer is 2. Notice that any node is part of its sub-tree.
 * Node 1 has a label 'b'. The sub-tree of node 1 contains nodes 1,4 and 5, as nodes 4 and 5 have different labels than node 1, the answer is just 1 (the node itself).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/01/q3e2.jpg" style="width: 300px; height: 253px;" />
 * Input: n = 4, edges = [[0,1],[1,2],[0,3]], labels = "bbbb"
 * Output: [4,2,1,1]
 * Explanation: The sub-tree of node 2 contains only node 2, so the answer is 1.
 * The sub-tree of node 3 contains only node 3, so the answer is 1.
 * The sub-tree of node 1 contains nodes 1 and 2, both have label 'b', thus the answer is 2.
 * The sub-tree of node 0 contains nodes 0, 1, 2 and 3, all with label 'b', thus the answer is 4.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/01/q3e3.jpg" style="width: 300px; height: 253px;" />
 * Input: n = 5, edges = [[0,1],[0,2],[1,3],[0,4]], labels = "aabab"
 * Output: [3,2,1,1,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	labels.length == n
 * 	labels is consisting of only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/
// discuss: https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut result = vec![0; edges.len() + 1];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n as usize]; //edges to node-based graph encoding for easy traversal of neighbours
        edges.into_iter().for_each(|edge| {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        });

        Self::dfs_visit_node(0, 0, &graph, &labels.as_bytes(), &mut result); //kick off the dfs at the root node
        result
    }

    fn dfs_visit_node(
        parent: usize,
        node: usize,
        graph: &Vec<Vec<usize>>,
        label_mask: &[u8],
        result: &mut Vec<i32>,
    ) -> Vec<i32> {
        fn label_to_idx(l: u8) -> usize {
            (l - b'a') as usize
        } //using this to convert every time, the compiler will hopefully inline

        let mut dp = graph[node].iter().filter(|n| { **n != parent }). //filtering the parent neighbour out
                fold(vec![0; 26], |mut fmap, cn| { //combine the frequency maps of all child sub-trees
                    fmap.iter_mut().zip(Self::dfs_visit_node(node, *cn, graph, label_mask, result).iter()).for_each(|(pf, cf)| {
                        *pf += cf;
                    });
                fmap
            });

        dp[label_to_idx(label_mask[node as usize])] += 1; // include the current node in the frequency counting
        result[node as usize] = dp[label_to_idx(label_mask[node as usize])]; // prep the final result for the current node
        dp //the result is the frequency map of the current node
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1519_example_1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let labels = "abaedcd".to_string();

        let result = vec![2, 1, 1, 1, 1, 1, 1];

        assert_eq!(Solution::count_sub_trees(n, edges, labels), result);
    }

    #[test]
    fn test_1519_example_2() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];
        let labels = "bbbb".to_string();

        let result = vec![4, 2, 1, 1];

        assert_eq!(Solution::count_sub_trees(n, edges, labels), result);
    }

    #[test]
    fn test_1519_example_3() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]];
        let labels = "aabab".to_string();

        let result = vec![3, 2, 1, 1, 1];

        assert_eq!(Solution::count_sub_trees(n, edges, labels), result);
    }
}
