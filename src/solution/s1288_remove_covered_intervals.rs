/**
 * [1288] Remove Covered Intervals
 *
 * Given an array intervals where intervals[i] = [li, ri] represent the interval [li, ri), remove all intervals that are covered by another interval in the list.
 * The interval [a, b) is covered by the interval [c, d) if and only if c <= a and b <= d.
 * Return the number of remaining intervals.
 *
 * Example 1:
 *
 * Input: intervals = [[1,4],[3,6],[2,8]]
 * Output: 2
 * Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
 *
 * Example 2:
 *
 * Input: intervals = [[1,4],[2,3]]
 * Output: 1
 *
 *
 * Constraints:
 *
 * 	1 <= intervals.length <= 1000
 * 	intervals[i].length == 2
 * 	0 <= li < ri <= 10^5
 * 	All the given intervals are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-covered-intervals/
// discuss: https://leetcode.com/problems/remove-covered-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|v: &Vec<i32>| (v[0], -v[1]));
        let mut result = 0;
        let mut end = 0;
        for interval in intervals.iter() {
            if interval[1] > end {
                result += 1;
                end = interval[1];
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1288_example_1() {
        let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
        let result = 2;

        assert_eq!(Solution::remove_covered_intervals(intervals), result);
    }

    #[test]
    fn test_1288_example_2() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let result = 1;

        assert_eq!(Solution::remove_covered_intervals(intervals), result);
    }
}
