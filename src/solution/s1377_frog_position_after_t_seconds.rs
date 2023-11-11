/**
 * [1377] Frog Position After T Seconds
 *
 * Given an undirected tree consisting of n vertices numbered from 1 to n. A frog starts jumping from vertex 1. In one second, the frog jumps from its current vertex to another unvisited vertex if they are directly connected. The frog can not jump back to a visited vertex. In case the frog can jump to several vertices, it jumps randomly to one of them with the same probability. Otherwise, when the frog can not jump to any unvisited vertex, it jumps forever on the same vertex.
 * The edges of the undirected tree are given in the array edges, where edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and bi.
 * Return the probability that after t seconds the frog is on the vertex target. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/frog1.jpg" style="width: 338px; height: 304px;" />
 * Input: n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 2, target = 4
 * Output: 0.16666666666666666
 * Explanation: The figure above shows the given graph. The frog starts at vertex 1, jumping with 1/3 probability to the vertex 2 after second 1 and then jumping with 1/2 probability to vertex 4 after second 2. Thus the probability for the frog is on the vertex 4 after 2 seconds is 1/3 * 1/2 = 1/6 = 0.16666666666666666.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/frog2.jpg" style="width: 304px; height: 304px;" />
 *
 * Input: n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 1, target = 7
 * Output: 0.3333333333333333
 * Explanation: The figure above shows the given graph. The frog starts at vertex 1, jumping with 1/3 = 0.3333333333333333 probability to the vertex 7 after second 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	1 <= ai, bi <= n
 * 	1 <= t <= 50
 * 	1 <= target <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/frog-position-after-t-seconds/
// discuss: https://leetcode.com/problems/frog-position-after-t-seconds/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        if n == 1 {
            return 1.0;
        }
        let mut g = vec![vec![]; n as usize + 1];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }
        let mut seen = vec![false; n as usize + 1];

        Self::dfs_helper(&g, target as usize, &mut seen, 1, t)
    }

    fn dfs_helper(
        g: &Vec<Vec<usize>>,
        target: usize,
        seen: &mut Vec<bool>,
        i: usize,
        t: i32,
    ) -> f64 {
        if i != 1 && g[i].len() == 1 || t == 0 {
            return if i == target { 1.0 } else { 0.0 };
        }
        seen[i] = true;
        let mut result = 0.0;
        for j in &g[i] {
            if !seen[*j] {
                result += Self::dfs_helper(g, target, seen, *j, t - 1);
            }
        }
        result / (g[i].len() - usize::from(i != 1)) as f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1377_example_1() {
        let n = 7;
        let edges = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 7],
            vec![2, 4],
            vec![2, 6],
            vec![3, 5],
        ];
        let t = 2;
        let target = 4;

        let result = 0.16666666666666666;

        assert_f64_near!(Solution::frog_position(n, edges, t, target), result);
    }

    #[test]
    fn test_1377_example_2() {
        let n = 7;
        let edges = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 7],
            vec![2, 4],
            vec![2, 6],
            vec![3, 5],
        ];
        let t = 1;
        let target = 7;

        let result = 0.3333333333333333;

        assert_f64_near!(Solution::frog_position(n, edges, t, target), result);
    }
}
