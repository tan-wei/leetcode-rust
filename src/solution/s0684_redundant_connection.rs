/**
 * [0684] Redundant Connection
 *
 * In this problem, a tree is an undirected graph that is connected and has no cycles.
 * You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
 * Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/reduntant1-1-graph.jpg" style="width: 222px; height: 222px;" />
 * Input: edges = [[1,2],[1,3],[2,3]]
 * Output: [2,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/reduntant1-2-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
 * Output: [1,4]
 *
 *  
 * Constraints:
 *
 * 	n == edges.length
 * 	3 <= n <= 1000
 * 	edges[i].length == 2
 * 	1 <= ai < bi <= edges.length
 * 	ai != bi
 * 	There are no repeated edges.
 * 	The given graph is connected.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/redundant-connection/
// discuss: https://leetcode.com/problems/redundant-connection/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tmp = vec![];
        let mut graph = vec![tmp; 1001];
        let mut result = vec![0; 2];

        for i in 0..edges.len() {
            let mut seen = std::collections::HashSet::new();
            if i != 0 && Self::is_connected(&graph, &mut seen, edges[i][0], edges[i][1]) {
                result[0] = edges[i][0];
                result[1] = edges[i][1];
            }

            graph[edges[i][0] as usize].push(edges[i][1]);
            graph[edges[i][1] as usize].push(edges[i][0]);
        }

        result
    }

    fn is_connected(
        graph: &Vec<Vec<i32>>,
        mut seen: &mut std::collections::HashSet<i32>,
        val: i32,
        target: i32,
    ) -> bool {
        if !seen.contains(&val) {
            seen.insert(val);
            if val == target {
                return true;
            }

            let index: usize = val as usize;
            for i in 0..graph[index].len() {
                if Self::is_connected(&graph, &mut seen, graph[index][i], target) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0684_example_1() {}
}
