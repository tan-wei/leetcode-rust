/**
 * [0991] Broken Calculator
 *
 * There is a broken calculator that has the integer startValue on its display initially. In one operation, you can:
 *
 * 	multiply the number on display by 2, or
 * 	subtract 1 from the number on display.
 *
 * Given two integers startValue and target, return the minimum number of operations needed to display target on the calculator.
 *  
 * Example 1:
 *
 * Input: startValue = 2, target = 3
 * Output: 2
 * Explanation: Use double operation and then decrement operation {2 -> 4 -> 3}.
 *
 * Example 2:
 *
 * Input: startValue = 5, target = 8
 * Output: 2
 * Explanation: Use decrement and then double {5 -> 4 -> 8}.
 *
 * Example 3:
 *
 * Input: startValue = 3, target = 10
 * Output: 3
 * Explanation: Use double, decrement and double {3 -> 6 -> 5 -> 10}.
 *
 *  
 * Constraints:
 *
 * 	1 <= startValue, target <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/broken-calculator/
// discuss: https://leetcode.com/problems/broken-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        if start_value >= target {
            start_value - target
        } else {
            1 + target % 2 + Self::broken_calc(start_value, (target + 1) / 2)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0991_example_1() {
        let start_value = 2;
        let target = 3;
        let result = 2;

        assert_eq!(Solution::broken_calc(start_value, target), result);
    }

    #[test]
    fn test_0991_example_2() {
        let start_value = 3;
        let target = 10;
        let result = 3;

        assert_eq!(Solution::broken_calc(start_value, target), result);
    }

    #[test]
    fn test_0991_example_3() {
        let start_value = 5;
        let target = 8;
        let result = 2;

        assert_eq!(Solution::broken_calc(start_value, target), result);
    }
}
