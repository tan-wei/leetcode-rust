/**
 * [1786] Number of Restricted Paths From First to Last Node
 *
 * There is an undirected weighted connected graph. You are given a positive integer n which denotes that the graph has n nodes labeled from 1 to n, and an array edges where each edges[i] = [ui, vi, weighti] denotes that there is an edge between nodes ui and vi with weight equal to weighti.
 * A path from node start to node end is a sequence of nodes [z0, z1, z2, ..., zk] such that z0 = start and zk = end and there is an edge between zi and zi+1 where 0 <= i <= k-1.
 * The distance of a path is the sum of the weights on the edges of the path. Let distanceToLastNode(x) denote the shortest distance of a path between node n and node x. A restricted path is a path that also satisfies that distanceToLastNode(zi) > distanceToLastNode(zi+1) where 0 <= i <= k-1.
 * Return the number of restricted paths from node 1 to node n. Since that number may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex1.png" style="width: 351px; height: 341px;" />
 * Input: n = 5, edges = [[1,2,3],[1,3,3],[2,3,1],[1,4,2],[5,2,2],[3,5,1],[5,4,10]]
 * Output: 3
 * Explanation: Each circle contains the node number in black and its distanceToLastNode value in blue. The three restricted paths are:
 * 1) 1 --> 2 --> 5
 * 2) 1 --> 2 --> 3 --> 5
 * 3) 1 --> 3 --> 5
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex22.png" style="width: 356px; height: 401px;" />
 * Input: n = 7, edges = [[1,3,1],[4,1,2],[7,3,4],[2,5,3],[5,6,1],[6,7,2],[7,5,3],[2,6,4]]
 * Output: 1
 * Explanation: Each circle contains the node number in black and its distanceToLastNode value in blue. The only restricted path is 1 --> 3 --> 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2 * 10^4
 * 	n - 1 <= edges.length <= 4 * 10^4
 * 	edges[i].length == 3
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	1 <= weighti <= 10^5
 * 	There is at most one edge between any two nodes.
 * 	There is at least one path between any two nodes.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-restricted-paths-from-first-to-last-node/
// discuss: https://leetcode.com/problems/number-of-restricted-paths-from-first-to-last-node/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-restricted-paths-from-first-to-last-node/solutions/1190377/rust-dijkstra-dfs-solution/
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let adjancency_list = {
            let mut res = vec![vec![]; n + 1];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = edge[2];
                res[u].push((v, w));
                res[v].push((u, w));
            }
            res
        };
        // Step1: use dijkstra to calc all shortest path from i to n.
        let mut distances = vec![std::i32::MAX; n + 1];
        distances[n] = 0;
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((std::cmp::Reverse(0), n));
        while let Some((std::cmp::Reverse(distance), u)) = pq.pop() {
            for &(v, w) in &adjancency_list[u] {
                let next_distance = distance + w;
                if next_distance < distances[v] {
                    distances[v] = next_distance;
                    pq.push((std::cmp::Reverse(next_distance), v));
                }
            }
        }
        // Step2: use top-down dp to calc numbers of paths
        let mut cache = std::collections::HashMap::new();
        Self::dfs_helper(&adjancency_list, &distances, 1, &mut cache) as i32
    }
    fn dfs_helper(
        adjancency_list: &[Vec<(usize, i32)>],
        distances: &[i32],
        u: usize,
        cache: &mut std::collections::HashMap<usize, i64>,
    ) -> i64 {
        if let Some(&cached) = cache.get(&u) {
            return cached;
        }
        let mut result = 0;
        for &(v, _) in &adjancency_list[u] {
            if distances[v] == 0 {
                result += 1;
            } else if distances[u] > distances[v] {
                result += Self::dfs_helper(adjancency_list, distances, v, cache);
            }
        }
        result %= 1_000_000_007;

        cache.insert(u, result);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1786_example_1() {
        let n = 5;
        let edges = vec![
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![2, 3, 1],
            vec![1, 4, 2],
            vec![5, 2, 2],
            vec![3, 5, 1],
            vec![5, 4, 10],
        ];

        let result = 3;

        assert_eq!(Solution::count_restricted_paths(n, edges), result);
    }

    #[test]
    fn test_1786_example_2() {
        let n = 7;
        let edges = vec![
            vec![1, 3, 1],
            vec![4, 1, 2],
            vec![7, 3, 4],
            vec![2, 5, 3],
            vec![5, 6, 1],
            vec![6, 7, 2],
            vec![7, 5, 3],
            vec![2, 6, 4],
        ];

        let result = 1;

        assert_eq!(Solution::count_restricted_paths(n, edges), result);
    }
}
