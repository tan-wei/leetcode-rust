/**
 * [0636] Exclusive Time of Functions
 *
 * On a single-threaded CPU, we execute a program containing n functions. Each function has a unique ID between 0 and n-1.
 * Function calls are stored in a <a href="https://en.wikipedia.org/wiki/Call_stack">call stack</a>: when a function call starts, its ID is pushed onto the stack, and when a function call ends, its ID is popped off the stack. The function whose ID is at the top of the stack is the current function being executed. Each time a function starts or ends, we write a log with the ID, whether it started or ended, and the timestamp.
 * You are given a list logs, where logs[i] represents the i^th log message formatted as a string "{function_id}:{"start" | "end"}:{timestamp}". For example, "0:start:3" means a function call with function ID 0 started at the beginning of timestamp 3, and "1:end:2" means a function call with function ID 1 ended at the end of timestamp 2. Note that a function can be called multiple times, possibly recursively.
 * A function's exclusive time is the sum of execution times for all function calls in the program. For example, if a function is called twice, one call executing for 2 time units and another call executing for 1 time unit, the exclusive time is 2 + 1 = 3.
 * Return the exclusive time of each function in an array, where the value at the i^th index represents the exclusive time for the function with ID i.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/05/diag1b.png" style="width: 550px; height: 239px;" />
 * Input: n = 2, logs = ["0:start:0","1:start:2","1:end:5","0:end:6"]
 * Output: [3,4]
 * Explanation:
 * Function 0 starts at the beginning of time 0, then it executes 2 for units of time and reaches the end of time 1.
 * Function 1 starts at the beginning of time 2, executes for 4 units of time, and ends at the end of time 5.
 * Function 0 resumes execution at the beginning of time 6 and executes for 1 unit of time.
 * So function 0 spends 2 + 1 = 3 units of total time executing, and function 1 spends 4 units of total time executing.
 *
 * Example 2:
 *
 * Input: n = 1, logs = ["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
 * Output: [8]
 * Explanation:
 * Function 0 starts at the beginning of time 0, executes for 2 units of time, and recursively calls itself.
 * Function 0 (recursive call) starts at the beginning of time 2 and executes for 4 units of time.
 * Function 0 (initial call) resumes execution then immediately calls itself again.
 * Function 0 (2nd recursive call) starts at the beginning of time 6 and executes for 1 unit of time.
 * Function 0 (initial call) resumes execution at the beginning of time 7 and executes for 1 unit of time.
 * So function 0 spends 2 + 4 + 1 + 1 = 8 units of total time executing.
 *
 * Example 3:
 *
 * Input: n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]
 * Output: [7,1]
 * Explanation:
 * Function 0 starts at the beginning of time 0, executes for 2 units of time, and recursively calls itself.
 * Function 0 (recursive call) starts at the beginning of time 2 and executes for 4 units of time.
 * Function 0 (initial call) resumes execution then immediately calls function 1.
 * Function 1 starts at the beginning of time 6, executes 1 unit of time, and ends at the end of time 6.
 * Function 0 resumes execution at the beginning of time 6 and executes for 2 units of time.
 * So function 0 spends 2 + 4 + 1 = 7 units of total time executing, and function 1 spends 1 unit of total time executing.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	1 <= logs.length <= 500
 * 	0 <= function_id < n
 * 	0 <= timestamp <= 10^9
 * 	No two start events will happen at the same timestamp.
 * 	No two end events will happen at the same timestamp.
 * 	Each function has an "end" log for each "start" log.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/exclusive-time-of-functions/
// discuss: https://leetcode.com/problems/exclusive-time-of-functions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut timeline = vec![0; n as usize];
        let mut stack = std::collections::VecDeque::new();
        let mut previous_timestamp = 0;

        for log in logs {
            let splited = log.split(":").collect::<Vec<&str>>();
            let id = splited[0].to_string().parse::<i32>().unwrap();
            let state = splited[1];
            let current_timestamp = splited[2].to_string().parse::<i32>().unwrap();

            if let Some(&index) = stack.back() {
                timeline[index as usize] += current_timestamp - previous_timestamp;
            }

            previous_timestamp = current_timestamp;

            if state == "start".to_string() {
                stack.push_back(id)
            } else {
                timeline[stack.pop_back().unwrap() as usize] += 1;
                previous_timestamp += 1;
            }
        }

        timeline
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0636_example_1() {
        let n = 2;
        let logs = vec_string!["0:start:0", "1:start:2", "1:end:5", "0:end:6"];
        let result = vec![3, 4];

        assert_eq!(Solution::exclusive_time(n, logs), result);
    }

    #[test]
    fn test_0636_example_2() {
        let n = 1;
        let logs = vec_string![
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "0:start:6",
            "0:end:6",
            "0:end:7"
        ];
        let result = vec![8];

        assert_eq!(Solution::exclusive_time(n, logs), result);
    }

    #[test]
    fn test_0636_example_3() {
        let n = 2;
        let logs = vec_string![
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "1:start:6",
            "1:end:6",
            "0:end:7"
        ];
        let result = vec![7, 1];

        assert_eq!(Solution::exclusive_time(n, logs), result);
    }
}
