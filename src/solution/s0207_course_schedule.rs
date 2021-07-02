/**
 * [207] Course Schedule
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
 *
 * 	For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
 *
 * Return true if you can finish all courses. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: true
 * Explanation: There are a total of 2 courses to take.
 * To take course 1 you should have finished course 0. So it is possible.
 *
 * Example 2:
 *
 * Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
 * Output: false
 * Explanation: There are a total of 2 courses to take.
 * To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
 *
 *  
 * Constraints:
 *
 * 	1 <= numCourses <= 10^5
 * 	0 <= prerequisites.length <= 5000
 * 	prerequisites[i].length == 2
 * 	0 <= ai, bi < numCourses
 * 	All the pairs prerequisites[i] are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule/
// discuss: https://leetcode.com/problems/course-schedule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut v = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            v[p[0] as usize].1.push(p[1] as usize);
            v[p[1] as usize].0 += 1;
        }
        let mut stack: Vec<usize> = Vec::new();
        for (i, e) in (0..).zip(v.iter()) {
            if e.0 == 0 {
                stack.push(i);
            }
        }
        let mut count = 0;
        while let Some(last) = stack.pop() {
            count += 1;
            for i in v[last].1.clone() {
                v[i].0 -= 1;
                if v[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        count == num_courses
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0207_example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let result = true;

        assert_eq!(Solution::can_finish(num_courses, prerequisites), result);
    }

    #[test]
    fn test_0207_example_2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        let result = false;

        assert_eq!(Solution::can_finish(num_courses, prerequisites), result);
    }
}
