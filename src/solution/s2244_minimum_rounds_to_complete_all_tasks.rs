/**
 * [2244] Minimum Rounds to Complete All Tasks
 *
 * You are given a 0-indexed integer array tasks, where tasks[i] represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the same difficulty level.
 * Return the minimum rounds required to complete all the tasks, or -1 if it is not possible to complete all the tasks.
 *  
 * Example 1:
 *
 * Input: tasks = [2,2,3,3,2,4,4,4,4,4]
 * Output: 4
 * Explanation: To complete all the tasks, a possible plan is:
 * - In the first round, you complete 3 tasks of difficulty level 2.
 * - In the second round, you complete 2 tasks of difficulty level 3.
 * - In the third round, you complete 3 tasks of difficulty level 4.
 * - In the fourth round, you complete 2 tasks of difficulty level 4.  
 * It can be shown that all the tasks cannot be completed in fewer than 4 rounds, so the answer is 4.
 *
 * Example 2:
 *
 * Input: tasks = [2,3,3]
 * Output: -1
 * Explanation: There is only 1 task of difficulty level 2, but in each round, you can only complete either 2 or 3 tasks of the same difficulty level. Hence, you cannot complete all the tasks, and the answer is -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= tasks.length <= 10^5
 * 	1 <= tasks[i] <= 10^9
 *
 *  
 * Note: This question is the same as <a href="https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/description/" target="_blank">2870: Minimum Number of Operations to Make Array Empty.</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/
// discuss: https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2244_example_1() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];

        let result = 4;

        assert_eq!(Solution::minimum_rounds(tasks), result);
    }

    #[test]
    #[ignore]
    fn test_2244_example_2() {
        let tasks = vec![2, 3, 3];

        let result = -1;

        assert_eq!(Solution::minimum_rounds(tasks), result);
    }
}
