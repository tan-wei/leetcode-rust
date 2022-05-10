/**
 * [0672] Bulb Switcher II
 *
 * There is a room with n bulbs labeled from 1 to n that all are turned on initially, and four buttons on the wall. Each of the four buttons has a different functionality where:
 *
 * 	Button 1: Flips the status of all the bulbs.
 * 	Button 2: Flips the status of all the bulbs with even labels (i.e., 2, 4, ...).
 * 	Button 3: Flips the status of all the bulbs with odd labels (i.e., 1, 3, ...).
 * 	Button 4: Flips the status of all the bulbs with a label j = 3k + 1 where k = 0, 1, 2, ... (i.e., 1, 4, 7, 10, ...).
 *
 * You must make exactly presses button presses in total. For each press, you may pick any of the four buttons to press.
 * Given the two integers n and presses, return the number of different possible statuses after performing all presses button presses.
 *  
 * Example 1:
 *
 * Input: n = 1, presses = 1
 * Output: 2
 * Explanation: Status can be:
 * - [off] by pressing button 1
 * - [on] by pressing button 2
 *
 * Example 2:
 *
 * Input: n = 2, presses = 1
 * Output: 3
 * Explanation: Status can be:
 * - [off, off] by pressing button 1
 * - [on, off] by pressing button 2
 * - [off, on] by pressing button 3
 *
 * Example 3:
 *
 * Input: n = 3, presses = 1
 * Output: 4
 * Explanation: Status can be:
 * - [off, off, off] by pressing button 1
 * - [off, on, off] by pressing button 2
 * - [on, off, on] by pressing button 3
 * - [off, on, on] by pressing button 4
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	0 <= presses <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bulb-switcher-ii/
// discuss: https://leetcode.com/problems/bulb-switcher-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match (n, presses) {
            (_, 0) => 1,
            (1, _) => 2,
            (2, 1) => 3,
            (2, _) => 4,
            (_, 1) => 4,
            (_, 2) => 7,
            (_, presses @ _) if presses > 3 => 8,
            (_, _) => 8,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0672_example_1() {
        let n = 1;
        let presses = 1;
        let result = 2;

        assert_eq!(Solution::flip_lights(n, presses), result);
    }

    #[test]
    fn test_0672_example_2() {
        let n = 2;
        let presses = 1;
        let result = 3;

        assert_eq!(Solution::flip_lights(n, presses), result);
    }

    #[test]
    fn test_0672_example_3() {
        let n = 3;
        let presses = 1;
        let result = 4;

        assert_eq!(Solution::flip_lights(n, presses), result);
    }
}
