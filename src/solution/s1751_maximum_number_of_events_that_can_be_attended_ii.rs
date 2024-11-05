/**
 * [1751] Maximum Number of Events That Can Be Attended II
 *
 * You are given an array of events where events[i] = [startDayi, endDayi, valuei]. The i^th event starts at startDayi and ends at endDayi, and if you attend this event, you will receive a value of valuei. You are also given an integer k which represents the maximum number of events you can attend.
 * You can only attend one event at a time. If you choose to attend an event, you must attend the entire event. Note that the end day is inclusive: that is, you cannot attend two events where one of them starts and the other ends on the same day.
 * Return the maximum sum of values that you can receive by attending events.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png" style="width: 400px; height: 103px;" />
 *
 * Input: events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
 * Output: 7
 * Explanation: Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png" style="width: 400px; height: 103px;" />
 *
 * Input: events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
 * Output: 10
 * Explanation: Choose event 2 for a total value of 10.
 * Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png" style="width: 400px; height: 126px;" />
 *
 * Input: events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
 * Output: 9
 * Explanation: Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.
 *  
 * Constraints:
 *
 * 	1 <= k <= events.length
 * 	1 <= k * events.length <= 10^6
 * 	1 <= startDayi <= endDayi <= 10^9
 * 	1 <= valuei <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/
// discuss: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/solutions/3766437/rust-33ms-10-3-mb/
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        let mut events = events;

        events.sort_unstable();

        let mut dp = vec![vec![0; n + 1]; (k + 1) as _];

        (0..n).rev().for_each(|i| {
            let j = events.partition_point(|event| event[0] <= events[i][1]);
            (1..(k + 1) as usize).for_each(|k| {
                dp[k][i] = std::cmp::max(dp[k][i + 1], events[i][2] + dp[k - 1][j]);
            });
        });

        dp[k as usize][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1751_example_1() {
        let events = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]];
        let k = 2;

        let result = 7;

        assert_eq!(Solution::max_value(events, k), result);
    }

    #[test]
    fn test_1751_example_2() {
        let events = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]];
        let k = 2;

        let result = 10;

        assert_eq!(Solution::max_value(events, k), result);
    }

    #[test]
    fn test_1751_example_3() {
        let events = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]];
        let k = 3;

        let result = 9;

        assert_eq!(Solution::max_value(events, k), result);
    }
}
