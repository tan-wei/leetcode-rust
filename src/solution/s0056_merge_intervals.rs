/**
 * [56] Merge Intervals
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
 *  
 * Example 1:
 *
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 *
 * Example 2:
 *
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= starti <= endi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|x, y| x[0].cmp(&y[0]));

        let mut res = Vec::new();

        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        for interval in intervals.into_iter().skip(1) {
            let (curr_start, curr_end) = (interval[0], interval[1]);
            if curr_start > end {
                res.push(vec![start, end]);
                start = curr_start;
                end = curr_end;
            } else if curr_end > end {
                end = curr_end;
            }
        }

        res.push(vec![start, end]);

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0056_example_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        assert_eq!(Solution::merge(intervals), result);
    }

    #[test]
    fn test_0056_example_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = vec![vec![1, 5]];

        assert_eq!(Solution::merge(intervals), result);
    }
}
