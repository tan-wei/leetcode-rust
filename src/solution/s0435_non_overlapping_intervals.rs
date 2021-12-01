/**
 * [0435] Non-overlapping Intervals
 *
 * Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
 *  
 * Example 1:
 *
 * Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
 * Output: 1
 * Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.
 *
 * Example 2:
 *
 * Input: intervals = [[1,2],[1,2],[1,2]]
 * Output: 2
 * Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.
 *
 * Example 3:
 *
 * Input: intervals = [[1,2],[2,3]]
 * Output: 0
 * Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
 *
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^5
 * 	intervals[i].length == 2
 * 	-5 * 10^4 <= starti < endi <= 5 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/non-overlapping-intervals/
// discuss: https://leetcode.com/problems/non-overlapping-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut end = i32::MIN;
        let mut result = 0;
        intervals.sort_by_key(|i| i[1]);
        for i in intervals {
            if i[0] >= end {
                end = i[1]
            } else {
                result += 1;
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
    fn test_0435_example_1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        let result = 1;

        assert_eq!(Solution::erase_overlap_intervals(intervals), result);
    }

    #[test]
    fn test_0435_example_2() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        let result = 2;

        assert_eq!(Solution::erase_overlap_intervals(intervals), result);
    }

    #[test]
    fn test_0435_example_3() {
        let intervals = vec![vec![1, 2], vec![2, 3]];
        let result = 0;

        assert_eq!(Solution::erase_overlap_intervals(intervals), result);
    }
}
