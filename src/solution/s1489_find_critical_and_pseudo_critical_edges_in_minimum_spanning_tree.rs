/**
 * [1489] Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
 *
 * Given a weighted undirected connected graph with n vertices numbered from 0 to n - 1, and an array edges where edges[i] = [ai, bi, weighti] represents a bidirectional and weighted edge between nodes ai and bi. A minimum spanning tree (MST) is a subset of the graph's edges that connects all vertices without cycles and with the minimum possible total edge weight.
 * Find all the critical and pseudo-critical edges in the given graph's minimum spanning tree (MST). An MST edge whose deletion from the graph would cause the MST weight to increase is called a critical edge. On the other hand, a pseudo-critical edge is that which can appear in some MSTs but not all.
 * Note that you can return the indices of the edges in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex1.png" style="width: 259px; height: 262px;" />
 *
 * Input: n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
 * Output: [[0,1],[2,3,4,5]]
 * Explanation: The figure above describes the graph.
 * The following figure shows all the possible MSTs:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/msts.png" style="width: 540px; height: 553px;" />
 * Notice that the two edges 0 and 1 appear in all MSTs, therefore they are critical edges, so we return them in the first list of the output.
 * The edges 2, 3, 4, and 5 are only part of some MSTs, therefore they are considered pseudo-critical edges. We add them to the second list of the output.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex2.png" style="width: 247px; height: 253px;" />
 *
 * Input: n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
 * Output: [[],[0,1,2,3]]
 * Explanation: We can observe that since all 4 edges have equal weight, choosing any 3 edges from the given 4 will yield an MST. Therefore all 4 edges are pseudo-critical.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 100
 * 	1 <= edges.length <= min(200, n * (n - 1) / 2)
 * 	edges[i].length == 3
 * 	0 <= ai < bi < n
 * 	1 <= weighti <= 1000
 * 	All pairs (ai, bi) are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/
// discuss: https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/solutions/3930836/rust-union-find-kruskal-s-algorithm/
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut s, mut t) = (
            std::collections::HashSet::new(),
            std::collections::HashSet::new(),
        );
        let (m, n) = (edges.len(), n as usize);
        let cost = Self::kruskal(&edges, m, m, n);

        for k in 0..edges.len() {
            if Self::kruskal(&edges, k, m, n) > cost {
                s.insert(k as i32);
            }
            if Self::kruskal(&edges, m, k, n) == cost {
                t.insert(k as i32);
            }
        }

        let mut result = vec![vec![]; 2];
        for i in &t {
            if s.contains(i) == false {
                result[1].push(*i);
            }
        }
        result[0] = s.into_iter().collect::<Vec<_>>();

        result
    }
    fn find(p: &mut Vec<usize>, i: usize) -> usize {
        if p[i] == i {
            return i;
        }
        p[i] = Self::find(p, p[i]);
        p[i]
    }

    fn kruskal(edges: &Vec<Vec<i32>>, exclude: usize, include: usize, n: usize) -> i32 {
        let mut p = (0..n).into_iter().collect::<Vec<_>>();
        let mut edges = edges.clone();
        let (mut result, mut cnt, m) = (0, 0, edges.len());

        if include < m {
            let (u, v) = (edges[include][0] as usize, edges[include][1] as usize);
            let (i, j) = (Self::find(&mut p, u), Self::find(&mut p, v));
            p[i] = j;
            cnt += 1;
            result = edges[include][2];
        }
        if exclude.min(include) < m {
            edges.remove(exclude.min(include));
        }
        edges.sort_by(|a, b| a[2].cmp(&b[2]));

        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            let (i, j) = (Self::find(&mut p, u), Self::find(&mut p, v));
            if i == j {
                continue;
            }
            p[i] = j;
            cnt += 1;
            result += e[2];
        }

        if cnt + 1 < n { i32::MAX } else { result }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1489_example_1() {
        let n = 5;
        let edges = vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![2, 3, 2],
            vec![0, 3, 2],
            vec![0, 4, 3],
            vec![3, 4, 3],
            vec![1, 4, 6],
        ];

        let result = vec![vec![0, 1], vec![2, 3, 4, 5]];

        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(n, edges),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_1489_example_2() {
        let n = 4;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]];

        let result = vec![vec![], vec![0, 1, 2, 3]];

        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(n, edges),
            result
        );
    }
}
