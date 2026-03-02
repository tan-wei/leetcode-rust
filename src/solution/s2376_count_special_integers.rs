/**
 * [2376] Count Special Integers
 *
 * We call a positive integer special if all of its digits are distinct.
 * Given a positive integer n, return the number of special integers that belong to the interval [1, n].
 *  
 * Example 1:
 *
 * Input: n = 20
 * Output: 19
 * Explanation: All the integers from 1 to 20, except 11, are special. Thus, there are 19 special integers.
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: 5
 * Explanation: All the integers from 1 to 5 are special.
 *
 * Example 3:
 *
 * Input: n = 135
 * Output: 110
 * Explanation: There are 110 integers from 1 to 135 that are special.
 * Some of the integers that are not special are: 22, 114, and 131.
 *  
 * Constraints:
 *
 * 	1 <= n <= 2 * 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-special-integers/
// discuss: https://leetcode.com/problems/count-special-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2376_example_1() {
        let n = 20;

        let result = 19;

        assert_eq!(Solution::count_special_numbers(n), result);
    }

    #[test]
    #[ignore]
    fn test_2376_example_2() {
        let n = 5;

        let result = 5;

        assert_eq!(Solution::count_special_numbers(n), result);
    }

    #[test]
    #[ignore]
    fn test_2376_example_3() {
        let n = 135;

        let result = 110;

        assert_eq!(Solution::count_special_numbers(n), result);
    }
}
