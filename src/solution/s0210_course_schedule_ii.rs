/**
 * [210] Course Schedule II
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
 *
 * 	For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
 *
 * Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
 *  
 * Example 1:
 *
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: [0,1]
 * Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
 *
 * Example 2:
 *
 * Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
 * Output: [0,2,1,3]
 * Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
 * So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
 *
 * Example 3:
 *
 * Input: numCourses = 1, prerequisites = []
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	1 <= numCourses <= 2000
 * 	0 <= prerequisites.length <= numCourses * (numCourses - 1)
 * 	prerequisites[i].length == 2
 * 	0 <= ai, bi < numCourses
 * 	ai != bi
 * 	All the pairs [ai, bi] are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-ii/
// discuss: https://leetcode.com/problems/course-schedule-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            v[p[0] as usize].0 += 1;
            v[p[1] as usize].1.push(p[0] as usize);
        }
        let mut stack = Vec::new();
        for (i, e) in (0..).zip(v.iter()) {
            if e.0 == 0 {
                stack.push(i);
            }
        }
        let mut result = Vec::with_capacity(num_courses as usize);
        while let Some(last) = stack.pop() {
            result.push(last as i32);
            for i in v[last].1.clone() {
                v[i].0 -= 1;
                if v[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        if result.len() == num_courses as usize {
            result
        } else {
            vec![]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0210_example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let result = vec![0, 1];

        assert_eq!(Solution::find_order(num_courses, prerequisites), result);
    }

    #[test]
    fn test_0210_example_2() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let result = vec![0, 2, 1, 3];

        assert_eq!(Solution::find_order(num_courses, prerequisites), result);
    }

    #[test]
    fn test_0210_example_3() {
        let num_courses = 1;
        let prerequisites = vec![];
        let result = vec![0];

        assert_eq!(Solution::find_order(num_courses, prerequisites), result);
    }
}
