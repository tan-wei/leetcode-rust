/**
 * [0739] Daily Temperatures
 *
 * Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the i^th day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
 *  
 * Example 1:
 * Input: temperatures = [73,74,75,71,69,72,76,73]
 * Output: [1,1,4,2,1,1,0,0]
 * Example 2:
 * Input: temperatures = [30,40,50,60]
 * Output: [1,1,1,0]
 * Example 3:
 * Input: temperatures = [30,60,90]
 * Output: [1,1,0]
 *  
 * Constraints:
 *
 * 	1 <= temperatures.length <= 10^5
 * 	30 <= temperatures[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/daily-temperatures/
// discuss: https://leetcode.com/problems/daily-temperatures/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<_> = temperatures
            .into_iter()
            .rev()
            .enumerate()
            .scan(Vec::new(), |s, (i, ti)| {
                while s.last().map_or(false, |&(_, tj)| ti >= tj) {
                    s.pop();
                }
                let result = s.last().map_or(0, |&(j, _)| i - j);
                s.push((i, ti));
                Some(result as _)
            })
            .collect();
        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0739_example_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = vec![1, 1, 4, 2, 1, 1, 0, 0];

        assert_eq!(Solution::daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_0739_example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let result = vec![1, 1, 1, 0];

        assert_eq!(Solution::daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_0739_example_3() {
        let temperatures = vec![30, 60, 90];
        let result = vec![1, 1, 0];

        assert_eq!(Solution::daily_temperatures(temperatures), result);
    }
}
