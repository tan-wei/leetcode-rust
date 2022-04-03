/**
 * [0621] Task Scheduler
 *
 * Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
 * However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.
 * Return the least number of units of times that the CPU will take to finish all the given tasks.
 *  
 * Example 1:
 *
 * Input: tasks = ["A","A","A","B","B","B"], n = 2
 * Output: 8
 * Explanation:
 * A -> B -> idle -> A -> B -> idle -> A -> B
 * There is at least 2 units of time between any two same tasks.
 *
 * Example 2:
 *
 * Input: tasks = ["A","A","A","B","B","B"], n = 0
 * Output: 6
 * Explanation: On this case any permutation of size 6 would work since n = 0.
 * ["A","A","A","B","B","B"]
 * ["A","B","A","B","A","B"]
 * ["B","B","B","A","A","A"]
 * ...
 * And so on.
 *
 * Example 3:
 *
 * Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
 * Output: 16
 * Explanation:
 * One possible solution is
 * A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
 *
 *  
 * Constraints:
 *
 * 	1 <= task.length <= 10^4
 * 	tasks[i] is upper-case English letter.
 * 	The integer n is in the range [0, 100].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/task-scheduler/
// discuss: https://leetcode.com/problems/task-scheduler/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut mp: [i32; 26] = [0; 26];

        for &task in tasks.iter() {
            mp[(task as u8 - b'A') as usize] += 1;
        }

        let (mut val, mut num) = (0, 0);
        for &v in mp.iter() {
            match v.cmp(&val) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => num += 1,
                std::cmp::Ordering::Greater => {
                    val = v;
                    num = 1;
                }
            }
        }
        ((val - 1) * (n + 1) + num).max(tasks.len() as i32)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0621_example_1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;
        let result = 8;

        assert_eq!(Solution::least_interval(tasks, n), result);
    }

    #[test]
    fn test_0621_example_2() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 0;
        let result = 6;

        assert_eq!(Solution::least_interval(tasks, n), result);
    }

    #[test]
    fn test_0621_example_3() {
        let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
        let n = 2;
        let result = 16;

        assert_eq!(Solution::least_interval(tasks, n), result);
    }
}
