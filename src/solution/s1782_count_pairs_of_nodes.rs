/**
 * [1782] Count Pairs Of Nodes
 *
 * You are given an undirected graph defined by an integer n, the number of nodes, and a 2D integer array edges, the edges in the graph, where edges[i] = [ui, vi] indicates that there is an undirected edge between ui and vi. You are also given an integer array queries.
 * Let incident(a, b) be defined as the number of edges that are connected to either node a or b.
 * The answer to the j^th query is the number of pairs of nodes (a, b) that satisfy both of the following conditions:
 *
 * 	a < b
 * 	incident(a, b) > queries[j]
 *
 * Return an array answers such that answers.length == queries.length and answers[j] is the answer of the j^th query.
 * Note that there can be multiple edges between the same two nodes.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/winword_2021-06-08_00-58-39.png" style="width: 529px; height: 305px;" />
 * Input: n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]], queries = [2,3]
 * Output: [6,5]
 * Explanation: The calculations for incident(a, b) are shown in the table above.
 * The answers for each of the queries are as follows:
 * - answers[0] = 6. All the pairs have an incident(a, b) value greater than 2.
 * - answers[1] = 5. All the pairs except (3, 4) have an incident(a, b) value greater than 3.
 *
 * Example 2:
 *
 * Input: n = 5, edges = [[1,5],[1,5],[3,4],[2,5],[1,3],[5,1],[2,3],[2,5]], queries = [1,2,3,4,5]
 * Output: [10,10,9,8,6]
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 2 * 10^4
 * 	1 <= edges.length <= 10^5
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	1 <= queries.length <= 20
 * 	0 <= queries[j] < edges.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-pairs-of-nodes/
// discuss: https://leetcode.com/problems/count-pairs-of-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-pairs-of-nodes/solutions/2618164/rust-sort-hashmap-o-n-log-n-q/
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut degree = vec![0; n];
        let mut hm = std::collections::HashMap::<(usize, usize), i32>::new();

        for e in edges {
            let (u, v) = (e[0].min(e[1]) as usize - 1, e[0].max(e[1]) as usize - 1);
            degree[u] += 1;
            degree[v] += 1;
            *hm.entry((u, v)).or_default() += 1;
        }

        let mut data = degree.clone();
        let mut result = vec![];

        data.sort();
        for q in queries {
            let (mut temp, mut j) = (0, n);

            for i in 0..n {
                while j > 0 && data[i] + data[j - 1] > q {
                    j -= 1;
                }
                temp += (n - j) as i32;

                if data[i] + data[i] > q {
                    temp -= 1;
                }
            }

            temp /= 2;

            for (key, val) in &hm {
                if degree[key.0] + degree[key.1] > q && degree[key.0] + degree[key.1] - val <= q {
                    temp -= 1;
                }
            }

            result.push(temp);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1782_example_1() {
        let n = 4;
        let edges = vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]];
        let queries = vec![2, 3];

        let result = vec![6, 5];

        assert_eq!(Solution::count_pairs(n, edges, queries), result);
    }

    #[test]
    fn test_1782_example_2() {
        let n = 5;
        let edges = vec![
            vec![1, 5],
            vec![1, 5],
            vec![3, 4],
            vec![2, 5],
            vec![1, 3],
            vec![5, 1],
            vec![2, 3],
            vec![2, 5],
        ];
        let queries = vec![1, 2, 3, 4, 5];

        let result = vec![10, 10, 9, 8, 6];

        assert_eq!(Solution::count_pairs(n, edges, queries), result);
    }
}
