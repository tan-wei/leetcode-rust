/**
 * [2224] Minimum Number of Operations to Convert Time
 *
 * You are given two strings current and correct representing two 24-hour times.
 * 24-hour times are formatted as "HH:MM", where HH is between 00 and 23, and MM is between 00 and 59. The earliest 24-hour time is 00:00, and the latest is 23:59.
 * In one operation you can increase the time current by 1, 5, 15, or 60 minutes. You can perform this operation any number of times.
 * Return the minimum number of operations needed to convert current to correct.
 *  
 * Example 1:
 *
 * Input: current = "02:30", correct = "04:35"
 * Output: 3
 * Explanation:
 * We can convert current to correct in 3 operations as follows:
 * - Add 60 minutes to current. current becomes "03:30".
 * - Add 60 minutes to current. current becomes "04:30".
 * - Add 5 minutes to current. current becomes "04:35".
 * It can be proven that it is not possible to convert current to correct in fewer than 3 operations.
 * Example 2:
 *
 * Input: current = "11:00", correct = "11:01"
 * Output: 1
 * Explanation: We only have to add one minute to current, so the minimum number of operations needed is 1.
 *
 *  
 * Constraints:
 *
 * 	current and correct are in the format "HH:MM"
 * 	current <= correct
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let operation = [60, 15, 5, 1];
        let minute = |s: String| -> i32 {
            s.split(":")
                .map(|x| x.to_string().parse().unwrap())
                .reduce(|x, y| 60 * x + y)
                .unwrap()
        };

        let mut delta_time = minute(correct) - minute(current);
        let mut result = 0;

        for k in operation {
            result += delta_time / k;
            delta_time %= k;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2224_example_1() {
        let current = "02:30".to_string();
        let correct = "04:35".to_string();

        let result = 3;

        assert_eq!(Solution::convert_time(current, correct), result);
    }

    #[test]
    fn test_2224_example_2() {
        let current = "11:00".to_string();
        let correct = "11:01".to_string();

        let result = 1;

        assert_eq!(Solution::convert_time(current, correct), result);
    }
}
