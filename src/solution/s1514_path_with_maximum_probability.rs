/**
 * [1514] Path with Maximum Probability
 *
 * You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
 * Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
 * If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex1.png" style="width: 187px; height: 186px;" />
 *
 * Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
 * Output: 0.25000
 * Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex2.png" style="width: 189px; height: 186px;" />
 *
 * Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
 * Output: 0.30000
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex3.png" style="width: 215px; height: 191px;" />
 *
 * Input: n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
 * Output: 0.00000
 * Explanation: There is no path between 0 and 2.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^4
 * 	0 <= start, end < n
 * 	start != end
 * 	0 <= a, b < n
 * 	a != b
 * 	0 <= succProb.length == edges.length <= 2*10^4
 * 	0 <= succProb[i] <= 1
 * 	There is at most one edge between every two nodes.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/path-with-maximum-probability/
// discuss: https://leetcode.com/problems/path-with-maximum-probability/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(PartialEq, PartialOrd)]
struct Pair(f64, i32);

impl Eq for Pair {}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let (mut e, mut pq, mut result) = (
            vec![vec![]; n as usize],
            std::collections::BinaryHeap::from([Pair(1f64, start_node)]),
            vec![0f64; n as usize],
        );
        for (a, b, p) in edges
            .iter()
            .zip(succ_prob.iter())
            .map(|(v, &p)| (v[0] as usize, v[1] as usize, p))
        {
            e[a].push((b, p));
            e[b].push((a, p));
        }
        while pq.len() > 0 {
            match pq.pop().unwrap() {
                Pair(prob, curr) => {
                    if curr == end_node {
                        break;
                    }
                    for &(x, p) in e[curr as usize].iter() {
                        let t = prob * p;
                        if t > result[x] {
                            result[x] = t;
                            pq.push(Pair(t, x as i32));
                        }
                    }
                }
                _ => (),
            }
        }

        result[end_node as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1514_example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.2];
        let start_node = 0;
        let end_node = 2;

        let result = 0.25000;

        assert_f64_near!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            result
        );
    }

    #[test]
    fn test_1514_example_2() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.3];
        let start_node = 0;
        let end_node = 2;

        let result = 0.30000;

        assert_f64_near!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            result
        );
    }

    #[test]
    fn test_1514_example_3() {
        let n = 3;
        let edges = vec![vec![0, 1]];
        let succ_prob = vec![0.5];
        let start_node = 0;
        let end_node = 2;

        let result = 0.00000;

        assert_f64_near!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            result
        );
    }
}
