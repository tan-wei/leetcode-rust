/**
 * [310] Minimum Height Trees
 *
 * A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.
 * Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).
 * Return a list of all MHTs' root labels. You can return the answer in any order.
 * The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/01/e1.jpg" style="width: 800px; height: 213px;" />
 * Input: n = 4, edges = [[1,0],[1,2],[1,3]]
 * Output: [1]
 * Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/01/e2.jpg" style="width: 800px; height: 321px;" />
 * Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
 * Output: [3,4]
 *
 * Example 3:
 *
 * Input: n = 1, edges = []
 * Output: [0]
 *
 * Example 4:
 *
 * Input: n = 2, edges = [[0,1]]
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2 * 10^4
 * 	edges.length == n - 1
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	All the pairs (ai, bi) are distinct.
 * 	The given input is guaranteed to be a tree and there will be no repeated edges.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-height-trees/
// discuss: https://leetcode.com/problems/minimum-height-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut degrees = vec![0; n as usize];
        let mut queue = std::collections::VecDeque::new();
        let mut graph = vec![Vec::new(); n as usize];
        let mut num_vertexes = n;

        edges.iter().for_each(|edge| {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        });

        for edge in edges {
            let first_vertex = edge[0] as usize;
            let second_vertex = edge[1] as usize;
            degrees[first_vertex] = degrees[first_vertex] + 1;
            degrees[second_vertex] = degrees[second_vertex] + 1;
        }

        degrees.iter().enumerate().for_each(|(idx, degree)| {
            if *degree == 1 {
                queue.push_back(idx)
            }
        });

        while num_vertexes > 2 {
            for _ in 0..queue.len() {
                let leaf_to_remove = queue.pop_front().unwrap() as usize;
                num_vertexes -= 1;

                degrees[leaf_to_remove] = degrees[leaf_to_remove] - 1;

                for adjacent_vertex in &graph[leaf_to_remove] {
                    degrees[*adjacent_vertex as usize] = degrees[*adjacent_vertex as usize] - 1;

                    if degrees[*adjacent_vertex as usize] == 1 {
                        queue.push_back(*adjacent_vertex as usize);
                    }
                }
            }
        }

        queue.iter().map(|val| *val as i32).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0310_example_1() {
        let n = 4;
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        let result = vec![1];

        assert_eq!(Solution::find_min_height_trees(n, edges), result);
    }

    #[test]
    fn test_0310_example_2() {
        let n = 6;
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        let result = vec![3, 4];

        assert_eq!(Solution::find_min_height_trees(n, edges), result);
    }

    #[test]
    fn test_0310_example_3() {
        let n = 1;
        let edges = vec![];
        let result = vec![0];

        assert_eq!(Solution::find_min_height_trees(n, edges), result);
    }

    #[test]
    fn test_0310_example_4() {
        let n = 2;
        let edges = vec![vec![0, 1]];
        let result = vec![0, 1];

        assert_eq!(Solution::find_min_height_trees(n, edges), result);
    }
}
