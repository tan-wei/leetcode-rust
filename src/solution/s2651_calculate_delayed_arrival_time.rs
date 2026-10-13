/**
 * [2651] Calculate Delayed Arrival Time
 *
 * You are given a positive integer arrivalTime denoting the arrival time of a train in hours, and another positive integer delayedTime denoting the amount of delay in hours.
 * Return the time when the train will arrive at the station.
 * Note that the time in this problem is in 24-hours format.
 *  
 * Example 1:
 *
 * Input: arrivalTime = 15, delayedTime = 5
 * Output: 20
 * Explanation: Arrival time of the train was 15:00 hours. It is delayed by 5 hours. Now it will reach at 15+5 = 20 (20:00 hours).
 *
 * Example 2:
 *
 * Input: arrivalTime = 13, delayedTime = 11
 * Output: 0
 * Explanation: Arrival time of the train was 13:00 hours. It is delayed by 11 hours. Now it will reach at 13+11=24 (Which is denoted by 00:00 in 24 hours format so return 0).
 *
 *  
 * Constraints:
 *
 * 	1 <= arrivaltime < 24
 * 	1 <= delayedTime <= 24
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/calculate-delayed-arrival-time/
// discuss: https://leetcode.com/problems/calculate-delayed-arrival-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2651_example_1() {
        let arrival_time = 15;
        let delayed_time = 5;

        let result = 20;

        assert_eq!(
            Solution::find_delayed_arrival_time(arrival_time, delayed_time),
            result
        );
    }

    #[test]
    fn test_2651_example_2() {
        let arrival_time = 13;
        let delayed_time = 11;

        let result = 0;

        assert_eq!(
            Solution::find_delayed_arrival_time(arrival_time, delayed_time),
            result
        );
    }
}
