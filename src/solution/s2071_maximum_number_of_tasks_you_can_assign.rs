/**
 * [2071] Maximum Number of Tasks You Can Assign
 *
 * You have n tasks and m workers. Each task has a strength requirement stored in a 0-indexed integer array tasks, with the i^th task requiring tasks[i] strength to complete. The strength of each worker is stored in a 0-indexed integer array workers, with the j^th worker having workers[j] strength. Each worker can only be assigned to a single task and must have a strength greater than or equal to the task's strength requirement (i.e., workers[j] >= tasks[i]).
 * Additionally, you have pills magical pills that will increase a worker's strength by strength. You can decide which workers receive the magical pills, however, you may only give each worker at most one magical pill.
 * Given the 0-indexed integer arrays tasks and workers and the integers pills and strength, return the maximum number of tasks that can be completed.
 *  
 * Example 1:
 *
 * Input: tasks = [<u>3</u>,<u>2</u>,<u>1</u>], workers = [<u>0</u>,<u>3</u>,<u>3</u>], pills = 1, strength = 1
 * Output: 3
 * Explanation:
 * We can assign the magical pill and tasks as follows:
 * - Give the magical pill to worker 0.
 * - Assign worker 0 to task 2 (0 + 1 >= 1)
 * - Assign worker 1 to task 1 (3 >= 2)
 * - Assign worker 2 to task 0 (3 >= 3)
 *
 * Example 2:
 *
 * Input: tasks = [<u>5</u>,4], workers = [<u>0</u>,0,0], pills = 1, strength = 5
 * Output: 1
 * Explanation:
 * We can assign the magical pill and tasks as follows:
 * - Give the magical pill to worker 0.
 * - Assign worker 0 to task 0 (0 + 5 >= 5)
 *
 * Example 3:
 *
 * Input: tasks = [<u>10</u>,<u>15</u>,30], workers = [<u>0</u>,<u>10</u>,10,10,10], pills = 3, strength = 10
 * Output: 2
 * Explanation:
 * We can assign the magical pills and tasks as follows:
 * - Give the magical pill to worker 0 and worker 1.
 * - Assign worker 0 to task 0 (0 + 10 >= 10)
 * - Assign worker 1 to task 1 (10 + 10 >= 15)
 * The last pill is not given because it will not make any worker strong enough for the last task.
 *
 *  
 * Constraints:
 *
 * 	n == tasks.length
 * 	m == workers.length
 * 	1 <= n, m <= 5 * 10^4
 * 	0 <= pills <= m
 * 	0 <= tasks[i], workers[j], strength <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/
// discuss: https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2071_example_1() {
        let tasks = vec![3, 2, 1];
        let workers = vec![0, 3, 3];
        let pills = 1;
        let strength = 1;

        let result = 3;

        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2071_example_2() {
        let tasks = vec![5, 4];
        let workers = vec![0, 0, 0];
        let pills = 1;
        let strength = 5;

        let result = 1;

        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2071_example_3() {
        let tasks = vec![10, 15, 30];
        let workers = vec![0, 10, 10, 10, 10];
        let pills = 2;
        let strength = 10;

        let result = 2;

        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            result
        );
    }
}
