/**
 * [1443] Minimum Time to Collect All Apples in a Tree
 *
 * Given an undirected tree consisting of n vertices numbered from 0 to n-1, which has some apples in their vertices. You spend 1 second to walk over one edge of the tree. Return the minimum time in seconds you have to spend to collect all apples in the tree, starting at vertex 0 and coming back to this vertex.
 * The edges of the undirected tree are given in the array edges, where edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and bi. Additionally, there is a boolean array hasApple, where hasApple[i] = true means that vertex i has an apple; otherwise, it does not have any apple.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_1.png" style="width: 300px; height: 212px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,true,true,false]
 * Output: 8
 * Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows.  
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_2.png" style="width: 300px; height: 212px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,false,true,false]
 * Output: 6
 * Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows.  
 *
 * Example 3:
 *
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,false,false,false,false,false]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	0 <= ai < bi <= n - 1
 * 	hasApple.length == n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
// discuss: https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        Self::dfs_helper(0, 0, &has_apple, &adj)
    }

    fn dfs_helper(current: usize, parent: usize, has_apple: &[bool], adj: &[Vec<usize>]) -> i32 {
        adj[current]
            .iter()
            .filter(|&&child| parent != child)
            .fold(0, |acc, &child| {
                let sum_child = Self::dfs_helper(child, current, has_apple, adj);
                acc + sum_child
                    + if sum_child > 0 || has_apple[child] {
                        2
                    } else {
                        0
                    }
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1443_example_1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, true, true, false];

        let result = 8;

        assert_eq!(Solution::min_time(n, edges, has_apple), result);
    }

    #[test]
    fn test_1443_example_2() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, false, true, false];

        let result = 6;

        assert_eq!(Solution::min_time(n, edges, has_apple), result);
    }

    #[test]
    fn test_1443_example_3() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, false, false, false, false, false];

        let result = 0;

        assert_eq!(Solution::min_time(n, edges, has_apple), result);
    }
}
