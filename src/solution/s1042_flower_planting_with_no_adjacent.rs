/**
 * [1042] Flower Planting With No Adjacent
 *
 * You have n gardens, labeled from 1 to n, and an array paths where paths[i] = [xi, yi] describes a bidirectional path between garden xi to garden yi. In each garden, you want to plant one of 4 types of flowers.
 * All gardens have at most 3 paths coming into or leaving it.
 * Your task is to choose a flower type for each garden such that, for any two gardens connected by a path, they have different types of flowers.
 * Return any such a choice as an array answer, where answer[i] is the type of flower planted in the (i+1)^th garden. The flower types are denoted 1, 2, 3, or 4. It is guaranteed an answer exists.
 *  
 * Example 1:
 *
 * Input: n = 3, paths = [[1,2],[2,3],[3,1]]
 * Output: [1,2,3]
 * Explanation:
 * Gardens 1 and 2 have different types.
 * Gardens 2 and 3 have different types.
 * Gardens 3 and 1 have different types.
 * Hence, [1,2,3] is a valid answer. Other valid answers include [1,2,4], [1,4,2], and [3,2,1].
 *
 * Example 2:
 *
 * Input: n = 4, paths = [[1,2],[3,4]]
 * Output: [1,2,1,2]
 *
 * Example 3:
 *
 * Input: n = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
 * Output: [1,2,3,4]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 * 	0 <= paths.length <= 2 * 10^4
 * 	paths[i].length == 2
 * 	1 <= xi, yi <= n
 * 	xi != yi
 * 	Every garden has at most 3 paths coming into or leaving it.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flower-planting-with-no-adjacent/
// discuss: https://leetcode.com/problems/flower-planting-with-no-adjacent/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for p in paths {
            let (u, v) = (p[0] as usize - 1, p[1] as usize - 1);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut result = vec![0; n];
        for i in 0..n {
            if result[i] != 0 {
                continue;
            }
            result[i] = 1;
            Self::dfs_helper(i, &mut result, &graph);
        }

        result
    }

    fn dfs_helper(u: usize, result: &mut Vec<i32>, graph: &Vec<Vec<usize>>) {
        let mut sk = vec![u];

        while let Some(u) = sk.pop() {
            for v in &graph[u] {
                if result[*v] != 0 {
                    continue;
                }

                let mut s = (1..=4)
                    .into_iter()
                    .collect::<std::collections::HashSet<i32>>();
                for w in &graph[*v] {
                    s.remove(&result[*w]);
                }

                let mut s = s.into_iter().collect::<Vec<i32>>();
                result[*v] = s.pop().unwrap();
                sk.push(*v);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1042_example_1() {
        let n = 3;
        let paths = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let result = vec![1, 2, 3];

        assert_eq!(Solution::garden_no_adj(n, paths), result);
    }

    #[test]
    #[ignore]
    fn test_1042_example_2() {
        let n = 4;
        let paths = vec![vec![1, 2], vec![3, 4]];
        let result = vec![1, 2, 1, 2];

        assert_eq!(Solution::garden_no_adj(n, paths), result);
    }

    #[test]
    #[ignore]
    fn test_1042_example_3() {
        let n = 3;
        let paths = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![1, 3],
            vec![2, 4],
        ];
        let result = vec![1, 2, 3, 4];

        assert_eq!(Solution::garden_no_adj(n, paths), result);
    }
}
