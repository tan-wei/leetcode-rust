/**
 * [2335] Minimum Amount of Time to Fill Cups
 *
 * You have a water dispenser that can dispense cold, warm, and hot water. Every second, you can either fill up 2 cups with different types of water, or 1 cup of any type of water.
 * You are given a 0-indexed integer array amount of length 3 where amount[0], amount[1], and amount[2] denote the number of cold, warm, and hot water cups you need to fill respectively. Return the minimum number of seconds needed to fill up all the cups.
 *  
 * Example 1:
 *
 * Input: amount = [1,4,2]
 * Output: 4
 * Explanation: One way to fill up the cups is:
 * Second 1: Fill up a cold cup and a warm cup.
 * Second 2: Fill up a warm cup and a hot cup.
 * Second 3: Fill up a warm cup and a hot cup.
 * Second 4: Fill up a warm cup.
 * It can be proven that 4 is the minimum number of seconds needed.
 *
 * Example 2:
 *
 * Input: amount = [5,4,4]
 * Output: 7
 * Explanation: One way to fill up the cups is:
 * Second 1: Fill up a cold cup, and a hot cup.
 * Second 2: Fill up a cold cup, and a warm cup.
 * Second 3: Fill up a cold cup, and a warm cup.
 * Second 4: Fill up a warm cup, and a hot cup.
 * Second 5: Fill up a cold cup, and a hot cup.
 * Second 6: Fill up a cold cup, and a warm cup.
 * Second 7: Fill up a hot cup.
 *
 * Example 3:
 *
 * Input: amount = [5,0,0]
 * Output: 5
 * Explanation: Every second, we fill up a cold cup.
 *
 *  
 * Constraints:
 *
 * 	amount.length == 3
 * 	0 <= amount[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/
// discuss: https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut seconds = 0;
        let mut amount = amount;

        loop {
            amount.sort_unstable();
            if amount.last().unwrap() == &0 {
                return seconds;
            }
            amount.iter_mut().rev().take(2).for_each(|cup| *cup -= 1);
            seconds += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2335_example_1() {
        let amount = vec![1, 4, 2];

        let result = 4;

        assert_eq!(Solution::fill_cups(amount), result);
    }

    #[test]
    fn test_2335_example_2() {
        let amount = vec![5, 4, 4];

        let result = 7;

        assert_eq!(Solution::fill_cups(amount), result);
    }

    #[test]
    fn test_2335_example_3() {
        let amount = vec![5, 0, 0];

        let result = 5;

        assert_eq!(Solution::fill_cups(amount), result);
    }
}
