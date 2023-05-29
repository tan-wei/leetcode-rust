/**
 * [1124] Longest Well-Performing Interval
 *
 * We are given hours, a list of the number of hours worked per day for a given employee.
 * A day is considered to be a tiring day if and only if the number of hours worked is (strictly) greater than 8.
 * A well-performing interval is an interval of days for which the number of tiring days is strictly larger than the number of non-tiring days.
 * Return the length of the longest well-performing interval.
 *  
 * Example 1:
 *
 * Input: hours = [9,9,6,0,6,6,9]
 * Output: 3
 * Explanation: The longest well-performing interval is [9,9,6].
 *
 * Example 2:
 *
 * Input: hours = [6,6,6]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= hours.length <= 10^4
 * 	0 <= hours[i] <= 16
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-well-performing-interval/
// discuss: https://leetcode.com/problems/longest-well-performing-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut score = 0;
        let n = hours.len();
        let mut seen = std::collections::HashMap::new();
        for (i, &hour) in hours.iter().enumerate().take(n) {
            score += if hour > 8 { 1 } else { -1 };
            if score > 0 {
                result = i + 1;
            } else {
                seen.entry(score).or_insert(i);
                if seen.contains_key(&(score - 1)) {
                    result = result.max(i - seen[&(score - 1)]);
                }
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1124_example_1() {
        let hours = vec![9, 9, 6, 0, 6, 6, 9];
        let result = 3;

        assert_eq!(Solution::longest_wpi(hours), result);
    }

    #[test]
    fn test_1124_example_2() {
        let hours = vec![6, 6, 6];
        let result = 0;

        assert_eq!(Solution::longest_wpi(hours), result);
    }
}
