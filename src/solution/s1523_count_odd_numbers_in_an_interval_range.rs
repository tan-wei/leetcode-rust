/**
 * [1523] Count Odd Numbers in an Interval Range
 *
 * Given two non-negative integers low and <font face="monospace">high</font>. Return the count of odd numbers between low and <font face="monospace">high</font> (inclusive).
 *
 *  
 * Example 1:
 *
 *
 * Input: low = 3, high = 7
 * Output: 3
 * Explanation: The odd numbers between 3 and 7 are [3,5,7].
 *
 * Example 2:
 *
 *
 * Input: low = 8, high = 10
 * Output: 1
 * Explanation: The odd numbers between 8 and 10 are [9].
 *
 *  
 * Constraints:
 *
 *
 * 	0 <= low <= high <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/
// discuss: https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high - low) / 2) + (low % 2 | high % 2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1523_example_1() {
        let low = 3;
        let high = 7;

        let result = 3;

        assert_eq!(Solution::count_odds(low, high), result);
    }

    #[test]
    fn test_1523_example_2() {
        let low = 8;
        let high = 10;

        let result = 1;

        assert_eq!(Solution::count_odds(low, high), result);
    }
}
