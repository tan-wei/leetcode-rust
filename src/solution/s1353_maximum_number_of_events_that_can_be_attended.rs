/**
 * [1353] Maximum Number of Events That Can Be Attended
 *
 * You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.
 * You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.
 * Return the maximum number of events you can attend.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/05/e1.png" style="width: 400px; height: 267px;" />
 * Input: events = [[1,2],[2,3],[3,4]]
 * Output: 3
 * Explanation: You can attend all the three events.
 * One way to attend them all is as shown.
 * Attend the first event on day 1.
 * Attend the second event on day 2.
 * Attend the third event on day 3.
 *
 * Example 2:
 *
 * Input: events= [[1,2],[2,3],[3,4],[1,2]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= events.length <= 10^5
 * 	events[i].length == 2
 * 	1 <= startDayi <= endDayi <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
// discuss: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut f = vec![0; events[events.len() - 1][1] as usize + 2];
        for (i, item) in f.iter_mut().enumerate() {
            *item = i as i32;
        }

        let mut result = 0;

        for it in events.iter() {
            let x = Self::find(it[0], &mut f);
            if x <= it[1] {
                result += 1;
                f[x as usize] = Self::find(x + 1, &mut f);
            }
        }

        result
    }

    fn find(x: i32, f: &mut Vec<i32>) -> i32 {
        if f[x as usize] == x {
            x
        } else {
            f[x as usize] = Self::find(f[x as usize], f);
            f[x as usize]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1353_example_1() {
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4]];

        let result = 3;

        assert_eq!(Solution::max_events(events), result);
    }

    #[test]
    fn test_1353_example_2() {
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]];

        let result = 4;

        assert_eq!(Solution::max_events(events), result);
    }
}
