/**
 * [0399] Evaluate Division
 *
 * You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.
 * You are also given some queries, where queries[j] = [Cj, Dj] represents the j^th query where you must find the answer for Cj / Dj = ?.
 * Return the answers to all queries. If a single answer cannot be determined, return -1.0.
 * Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.
 *  
 * Example 1:
 *
 * Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
 * Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
 * Explanation:
 * Given: a / b = 2.0, b / c = 3.0
 * queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
 * return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
 *
 * Example 2:
 *
 * Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
 * Output: [3.75000,0.40000,5.00000,0.20000]
 *
 * Example 3:
 *
 * Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
 * Output: [0.50000,2.00000,-1.00000,-1.00000]
 *
 *  
 * Constraints:
 *
 * 	1 <= equations.length <= 20
 * 	equations[i].length == 2
 * 	1 <= Ai.length, Bi.length <= 5
 * 	values.length == equations.length
 * 	0.0 < values[i] <= 20.0
 * 	1 <= queries.length <= 20
 * 	queries[i].length == 2
 * 	1 <= Cj.length, Dj.length <= 5
 * 	Ai, Bi, Cj, Dj consist of lower case English letters and digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-division/
// discuss: https://leetcode.com/problems/evaluate-division/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/evaluate-division/discuss/799642/Rust-cheapest-and-best
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph = std::collections::HashMap::new();
        for i in 0..equations.len() {
            (*graph.entry(&equations[i][0]).or_insert(vec![])).push((values[i], &equations[i][1]));
            (*graph.entry(&equations[i][1]).or_insert(vec![]))
                .push((1.0 / values[i], &equations[i][0]));
        }
        queries
            .into_iter()
            .map(|v| {
                let mut seen = std::collections::HashSet::new();
                let mut q = std::collections::VecDeque::new();
                if let Some(edges) = graph.get(&v[0]) {
                    for edge in edges {
                        q.push_back(edge.clone());
                    }
                }
                while let Some((val, s)) = q.pop_front() {
                    seen.insert(s.to_owned());
                    if s == &v[1] {
                        return val;
                    } else {
                        for edge in graph.get(&s).unwrap_or(&vec![]) {
                            if !seen.contains(edge.1) {
                                q.push_back((val * edge.0, edge.1));
                            }
                        }
                    }
                }
                -1.0
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0399_example_1() {
        let equations = vec![vec_string!["a", "b"], vec_string!["b", "c"]];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec_string!["a", "c"],
            vec_string!["b", "a"],
            vec_string!["a", "e"],
            vec_string!["a", "a"],
            vec_string!["x", "x"],
        ];
        let result = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];

        assert_eq!(Solution::calc_equation(equations, values, queries), result);
    }

    #[test]
    fn test_0399_example_2() {
        let equations = vec![
            vec_string!["a", "b"],
            vec_string!["b", "c"],
            vec_string!["bc", "cd"],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec_string!["a", "c"],
            vec_string!["c", "b"],
            vec_string!["bc", "cd"],
            vec_string!["cd", "bc"],
        ];
        let result = vec![3.75000, 0.40000, 5.00000, 0.20000];

        assert_eq!(Solution::calc_equation(equations, values, queries), result);
    }

    #[test]
    fn test_0399_example_3() {
        let equations = vec![
            vec_string!["a", "b"],
            vec_string!["b", "c"],
            vec_string!["bc", "cd"],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec_string!["a", "c"],
            vec_string!["c", "b"],
            vec_string!["bc", "cd"],
            vec_string!["cd", "bc"],
        ];
        let result = vec![3.75000, 0.40000, 5.00000, 0.20000];

        assert_eq!(Solution::calc_equation(equations, values, queries), result);
    }
}
