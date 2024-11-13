/**
 * [1761] Minimum Degree of a Connected Trio in a Graph
 *
 * You are given an undirected graph. You are given an integer n which is the number of nodes in the graph and an array edges, where each edges[i] = [ui, vi] indicates that there is an undirected edge between ui and vi.
 * A connected trio is a set of three nodes where there is an edge between every pair of them.
 * The degree of a connected trio is the number of edges where one endpoint is in the trio, and the other is not.
 * Return the minimum degree of a connected trio in the graph, or -1 if the graph has no connected trios.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios1.png" style="width: 388px; height: 164px;" />
 * Input: n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
 * Output: 3
 * Explanation: There is exactly one trio, which is [1,2,3]. The edges that form its degree are bolded in the figure above.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios2.png" style="width: 388px; height: 164px;" />
 * Input: n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]
 * Output: 0
 * Explanation: There are exactly three trios:
 * 1) [1,4,3] with degree 0.
 * 2) [2,5,6] with degree 2.
 * 3) [5,6,7] with degree 2.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 400
 * 	edges[i].length == 2
 * 	1 <= edges.length <= n * (n-1) / 2
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	There are no repeated edges.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/
// discuss: https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/solutions/3209233/just-a-runnable-solution/
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut al = vec![vec![]; n + 1];
        let mut cnt = vec![0; n + 1];
        let mut result = std::i32::MAX;

        for e in edges.iter() {
            let t1 = e[0].min(e[1]) as usize;
            let t2 = e[0].max(e[1]) as usize;
            al[t1].push(t2);
            cnt[t1] += 1;
            cnt[t2] += 1;
        }

        for t1 in 1..=n {
            for &t2 in al[t1].iter() {
                for &t3 in al[t1].iter() {
                    if t2 < t3 && al[t2].contains(&t3) {
                        result = result.min(cnt[t1] + cnt[t2] + cnt[t3] - 6);
                    }
                }
            }
        }

        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1761_example_1() {
        let n = 6;
        let edges = vec![
            vec![1, 2],
            vec![1, 3],
            vec![3, 2],
            vec![4, 1],
            vec![5, 2],
            vec![3, 6],
        ];

        let result = 3;

        assert_eq!(Solution::min_trio_degree(n, edges), result);
    }

    #[test]
    fn test_1761_example_2() {
        let n = 7;
        let edges = vec![
            vec![1, 3],
            vec![4, 1],
            vec![4, 3],
            vec![2, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 5],
            vec![2, 6],
        ];

        let result = 0;

        assert_eq!(Solution::min_trio_degree(n, edges), result);
    }
}
