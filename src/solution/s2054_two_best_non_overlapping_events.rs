/**
 * [2054] Two Best Non-Overlapping Events
 *
 * You are given a 0-indexed 2D integer array of events where events[i] = [startTimei, endTimei, valuei]. The i^th event starts at startTimei and ends at endTimei, and if you attend this event, you will receive a value of valuei. You can choose at most two non-overlapping events to attend such that the sum of their values is maximized.
 * Return this maximum sum.
 * Note that the start time and end time is inclusive: that is, you cannot attend two events where one of them starts and the other ends at the same time. More specifically, if you attend an event with end time t, the next event must start at or after t + 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/21/picture5.png" style="width: 400px; height: 75px;" />
 * Input: events = [[1,3,2],[4,5,2],[2,4,3]]
 * Output: 4
 * Explanation: Choose the green events, 0 and 1 for a sum of 2 + 2 = 4.
 *
 * Example 2:
 * <img alt="Example 1 Diagram" src="https://assets.leetcode.com/uploads/2021/09/21/picture1.png" style="width: 400px; height: 77px;" />
 * Input: events = [[1,3,2],[4,5,2],[1,5,5]]
 * Output: 5
 * Explanation: Choose event 2 for a sum of 5.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/21/picture3.png" style="width: 400px; height: 66px;" />
 * Input: events = [[1,5,3],[1,5,1],[6,6,5]]
 * Output: 8
 * Explanation: Choose events 0 and 2 for a sum of 3 + 5 = 8.
 *  
 * Constraints:
 *
 * 	2 <= events.length <= 10^5
 * 	events[i].length == 3
 * 	1 <= startTimei <= endTimei <= 10^9
 * 	1 <= valuei <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-best-non-overlapping-events/
// discuss: https://leetcode.com/problems/two-best-non-overlapping-events/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events
            .into_iter()
            .flat_map(|event| [(event[0], 1, event[2]), (event[1] + 1, 0, event[2])])
            .collect::<Vec<_>>();

        events.sort_unstable();

        events
            .into_iter()
            .scan((0, 0), |state, (t, s, v)| {
                let ans = state.0;
                let max_value = state.1;
                if s == 1 {
                    let ans = ans.max(v + max_value);
                    *state = (ans, max_value);
                    Some(ans)
                } else {
                    let max_value = max_value.max(v);
                    *state = (ans, max_value);
                    Some(ans)
                }
            })
            .last()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2054_example_1() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]];

        let result = 4;

        assert_eq!(Solution::max_two_events(events), result);
    }

    #[test]
    fn test_2054_example_2() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]];

        let result = 5;

        assert_eq!(Solution::max_two_events(events), result);
    }

    #[test]
    fn test_2054_example_3() {
        let events = vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]];

        let result = 8;

        assert_eq!(Solution::max_two_events(events), result);
    }
}
