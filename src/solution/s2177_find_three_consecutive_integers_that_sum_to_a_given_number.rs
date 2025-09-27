/**
 * [2177] Find Three Consecutive Integers That Sum to a Given Number
 *
 * Given an integer num, return three consecutive integers (as a sorted array) that sum to num. If num cannot be expressed as the sum of three consecutive integers, return an empty array.
 *  
 * Example 1:
 *
 * Input: num = 33
 * Output: [10,11,12]
 * Explanation: 33 can be expressed as 10 + 11 + 12 = 33.
 * 10, 11, 12 are 3 consecutive integers, so we return [10, 11, 12].
 *
 * Example 2:
 *
 * Input: num = 4
 * Output: []
 * Explanation: There is no way to express 4 as the sum of 3 consecutive integers.
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 10^15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/
// discuss: https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        match num % 3 {
            0 => vec![num / 3 - 1, num / 3, num / 3 + 1],
            _ => Vec::new(),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2177_example_1() {
        let num = 33;

        let result = vec![10, 11, 12];

        assert_eq!(Solution::sum_of_three(num), result);
    }

    #[test]
    fn test_2177_example_2() {
        let num = 4;

        let result: Vec<i64> = vec![];

        assert_eq!(Solution::sum_of_three(num), result);
    }
}
