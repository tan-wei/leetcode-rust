/**
 * [1964] Find the Longest Valid Obstacle Course at Each Position
 *
 * You want to build some obstacle courses. You are given a 0-indexed integer array obstacles of length n, where obstacles[i] describes the height of the i^th obstacle.
 * For every index i between 0 and n - 1 (inclusive), find the length of the longest obstacle course in obstacles such that:
 *
 * 	You choose any number of obstacles between 0 and i inclusive.
 * 	You must include the i^th obstacle in the course.
 * 	You must put the chosen obstacles in the same order as they appear in obstacles.
 * 	Every obstacle (except the first) is taller than or the same height as the obstacle immediately before it.
 *
 * Return an array ans of length n, where ans[i] is the length of the longest obstacle course for index i as described above.
 *  
 * Example 1:
 *
 * Input: obstacles = [1,2,3,2]
 * Output: [1,2,3,3]
 * Explanation: The longest valid obstacle course at each position is:
 * - i = 0: [<u>1</u>], [1] has length 1.
 * - i = 1: [<u>1</u>,<u>2</u>], [1,2] has length 2.
 * - i = 2: [<u>1</u>,<u>2</u>,<u>3</u>], [1,2,3] has length 3.
 * - i = 3: [<u>1</u>,<u>2</u>,3,<u>2</u>], [1,2,2] has length 3.
 *
 * Example 2:
 *
 * Input: obstacles = [2,2,1]
 * Output: [1,2,1]
 * Explanation: The longest valid obstacle course at each position is:
 * - i = 0: [<u>2</u>], [2] has length 1.
 * - i = 1: [<u>2</u>,<u>2</u>], [2,2] has length 2.
 * - i = 2: [2,2,<u>1</u>], [1] has length 1.
 *
 * Example 3:
 *
 * Input: obstacles = [3,1,5,6,4,2]
 * Output: [1,1,2,3,2,2]
 * Explanation: The longest valid obstacle course at each position is:
 * - i = 0: [<u>3</u>], [3] has length 1.
 * - i = 1: [3,<u>1</u>], [1] has length 1.
 * - i = 2: [<u>3</u>,1,<u>5</u>], [3,5] has length 2. [1,5] is also valid.
 * - i = 3: [<u>3</u>,1,<u>5</u>,<u>6</u>], [3,5,6] has length 3. [1,5,6] is also valid.
 * - i = 4: [<u>3</u>,1,5,6,<u>4</u>], [3,4] has length 2. [1,4] is also valid.
 * - i = 5: [3,<u>1</u>,5,6,4,<u>2</u>], [1,2] has length 2.
 *
 *  
 * Constraints:
 *
 * 	n == obstacles.length
 * 	1 <= n <= 10^5
 * 	1 <= obstacles[i] <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/
// discuss: https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1964_example_1() {
        let obstacles = vec![1, 2, 3, 2];

        let result = vec![1, 2, 3, 3];

        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_1964_example_2() {
        let obstacles = vec![2, 2, 1];

        let result = vec![1, 2, 1];

        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_1964_example_3() {
        let obstacles = vec![3, 1, 5, 6, 4, 2];

        let result = vec![1, 1, 2, 3, 2, 2];

        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(obstacles),
            result
        );
    }
}
