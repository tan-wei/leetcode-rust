/**
 * [57] Insert Interval
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 * You may assume that the intervals were initially sorted according to their start times.
 *  
 * Example 1:
 *
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 *
 * Example 2:
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 * Example 3:
 *
 * Input: intervals = [], newInterval = [5,7]
 * Output: [[5,7]]
 *
 * Example 4:
 *
 * Input: intervals = [[1,5]], newInterval = [2,3]
 * Output: [[1,5]]
 *
 * Example 5:
 *
 * Input: intervals = [[1,5]], newInterval = [2,7]
 * Output: [[1,7]]
 *
 *  
 * Constraints:
 *
 * 	0 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= intervals[i][0] <= intervals[i][1] <= 10^5
 * 	intervals is sorted by intervals[i][0] in ascending order.
 * 	newInterval.length == 2
 * 	0 <= newInterval[0] <= newInterval[1] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/insert-interval/
// discuss: https://leetcode.com/problems/insert-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/insert-interval/discuss/844818/Rust-solution
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }
        let mut new_interval = new_interval;
        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
            new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);
        while i < intervals.len() {
            result.push(intervals[i].clone());
            i += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0057_example_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = vec![vec![1, 5], vec![6, 9]];

        assert_eq!(Solution::insert(intervals, new_interval), result);
    }

    #[test]
    fn test_0057_example_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let result = vec![vec![1, 2], vec![3, 10], vec![12, 16]];

        assert_eq!(Solution::insert(intervals, new_interval), result);
    }

    #[test]
    fn test_0057_example_3() {
        let intervals = vec![];
        let new_interval = vec![5, 7];
        let result = vec![vec![5, 7]];

        assert_eq!(Solution::insert(intervals, new_interval), result);
    }

    #[test]
    fn test_0057_example_4() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];
        let result = vec![vec![1, 5]];

        assert_eq!(Solution::insert(intervals, new_interval), result);
    }

    #[test]
    fn test_0057_example_5() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 7];
        let result = vec![vec![1, 7]];

        assert_eq!(Solution::insert(intervals, new_interval), result);
    }
}
