/**
 * [1462] Course Schedule IV
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course ai first if you want to take course bi.
 *
 * 	For example, the pair [0, 1] indicates that you have to take course 0 before you can take course 1.
 *
 * Prerequisites can also be indirect. If course a is a prerequisite of course b, and course b is a prerequisite of course c, then course a is a prerequisite of course c.
 * You are also given an array queries where queries[j] = [uj, vj]. For the j^th query, you should answer whether course uj is a prerequisite of course vj or not.
 * Return a boolean array answer, where answer[j] is the answer to the j^th query.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/01/courses4-1-graph.jpg" style="width: 222px; height: 62px;" />
 * Input: numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
 * Output: [false,true]
 * Explanation: The pair [1, 0] indicates that you have to take course 1 before you can take course 0.
 * Course 0 is not a prerequisite of course 1, but the opposite is true.
 *
 * Example 2:
 *
 * Input: numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
 * Output: [false,false]
 * Explanation: There are no prerequisites, and each course is independent.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/01/courses4-3-graph.jpg" style="width: 222px; height: 222px;" />
 * Input: numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
 * Output: [true,true]
 *
 *  
 * Constraints:
 *
 * 	2 <= numCourses <= 100
 * 	0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)
 * 	prerequisites[i].length == 2
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 * 	All the pairs [ai, bi] are unique.
 * 	The prerequisites graph has no cycles.
 * 	1 <= queries.length <= 10^4
 * 	0 <= ui, vi <= n - 1
 * 	ui != vi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-iv/
// discuss: https://leetcode.com/problems/course-schedule-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut g = vec![vec![]; n];
        for arr in prerequisites {
            let a = arr[0] as usize;
            let b = arr[1] as usize;
            g[b].push(a);
        }
        let mut memo = vec![None; n];

        for i in 0..n {
            if memo[i].is_none() {
                Self::dfs_helper(&g, &mut memo, i);
            }
        }

        let m = queries.len();
        let mut result = vec![false; m];
        for i in 0..m {
            let a = queries[i][0] as usize;
            let b = queries[i][1] as usize;
            if let Some(set) = &memo[b] {
                result[i] = set.contains(&a);
            }
        }

        result
    }

    fn dfs_helper(
        g: &Vec<Vec<usize>>,
        memo: &mut Vec<Option<std::collections::HashSet<usize>>>,
        ci: usize,
    ) -> std::collections::HashSet<usize> {
        if let Some(set) = &memo[ci] {
            return set.clone();
        }
        let mut set = std::collections::HashSet::new();
        for &ni in &g[ci] {
            let a_set = Self::dfs_helper(g, memo, ni);
            for v in a_set {
                set.insert(v);
            }
        }
        set.insert(ci);
        memo[ci] = Some(set.clone());
        set
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1462_example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];

        let result = vec![false, true];

        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            result
        );
    }

    #[test]
    fn test_1462_example_2() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![0, 1], vec![1, 0]];

        let result = vec![false, false];

        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            result
        );
    }

    #[test]
    fn test_1462_example_3() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];

        let queries = vec![vec![1, 0], vec![1, 2]];

        let result = vec![true, true];

        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            result
        );
    }
}
