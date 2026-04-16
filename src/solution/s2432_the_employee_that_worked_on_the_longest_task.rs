/**
 * [2432] The Employee That Worked on the Longest Task
 *
 * There are n employees, each with a unique id from 0 to n - 1.
 * You are given a 2D integer array logs where logs[i] = [idi, leaveTimei] where:
 *
 * 	idi is the id of the employee that worked on the i^th task, and
 * 	leaveTimei is the time at which the employee finished the i^th task. All the values leaveTimei are unique.
 *
 * Note that the i^th task starts the moment right after the (i - 1)^th task ends, and the 0^th task starts at time 0.
 * Return the id of the employee that worked the task with the longest time. If there is a tie between two or more employees, return the smallest id among them.
 *  
 * Example 1:
 *
 * Input: n = 10, logs = [[0,3],[2,5],[0,9],[1,15]]
 * Output: 1
 * Explanation:
 * Task 0 started at 0 and ended at 3 with 3 units of times.
 * Task 1 started at 3 and ended at 5 with 2 units of times.
 * Task 2 started at 5 and ended at 9 with 4 units of times.
 * Task 3 started at 9 and ended at 15 with 6 units of times.
 * The task with the longest time is task 3 and the employee with id 1 is the one that worked on it, so we return 1.
 *
 * Example 2:
 *
 * Input: n = 26, logs = [[1,1],[3,7],[2,12],[7,17]]
 * Output: 3
 * Explanation:
 * Task 0 started at 0 and ended at 1 with 1 unit of times.
 * Task 1 started at 1 and ended at 7 with 6 units of times.
 * Task 2 started at 7 and ended at 12 with 5 units of times.
 * Task 3 started at 12 and ended at 17 with 5 units of times.
 * The tasks with the longest time is task 1. The employee that worked on it is 3, so we return 3.
 *
 * Example 3:
 *
 * Input: n = 2, logs = [[0,10],[1,20]]
 * Output: 0
 * Explanation:
 * Task 0 started at 0 and ended at 10 with 10 units of times.
 * Task 1 started at 10 and ended at 20 with 10 units of times.
 * The tasks with the longest time are tasks 0 and 1. The employees that worked on them are 0 and 1, so we return the smallest id 0.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 500
 * 	1 <= logs.length <= 500
 * 	logs[i].length == 2
 * 	0 <= idi <= n - 1
 * 	1 <= leaveTimei <= 500
 * 	idi != idi+1
 * 	leaveTimei are sorted in a strictly increasing order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/
// discuss: https://leetcode.com/problems/the-employee-that-worked-on-the-longest-task/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut last_end = 0;
        logs.into_iter()
            .map(|user_taskstop| {
                let dur = user_taskstop[1] - last_end;
                last_end = user_taskstop[1];
                (dur, -user_taskstop[0])
            })
            .max()
            .unwrap()
            .1
            * -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2432_example_1() {
        let n = 10;
        let logs = vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]];

        let result = 1;

        assert_eq!(Solution::hardest_worker(n, logs), result);
    }

    #[test]
    fn test_2432_example_2() {
        let n = 26;
        let logs = vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]];

        let result = 3;

        assert_eq!(Solution::hardest_worker(n, logs), result);
    }

    #[test]
    fn test_2432_example_3() {
        let n = 2;
        let logs = vec![vec![0, 10], vec![1, 20]];

        let result = 0;

        assert_eq!(Solution::hardest_worker(n, logs), result);
    }
}
