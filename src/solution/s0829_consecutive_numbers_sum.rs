/**
 * [0829] Consecutive Numbers Sum
 *
 * Given an integer n, return the number of ways you can write n as the sum of consecutive positive integers.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 2
 * Explanation: 5 = 2 + 3
 *
 * Example 2:
 *
 * Input: n = 9
 * Output: 3
 * Explanation: 9 = 4 + 5 = 2 + 3 + 4
 *
 * Example 3:
 *
 * Input: n = 15
 * Output: 4
 * Explanation: 15 = 8 + 7 = 4 + 5 + 6 = 1 + 2 + 3 + 4 + 5
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/consecutive-numbers-sum/
// discuss: https://leetcode.com/problems/consecutive-numbers-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        (1..)
            .take_while(|&i| i * i < n * 2)
            .filter(|&i| {
                if i % 2 == 0 {
                    n % i != 0 && (2 * n) % i == 0
                } else {
                    n % i == 0
                }
            })
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0829_example_1() {
        let n = 5;
        let result = 2;

        assert_eq!(Solution::consecutive_numbers_sum(n), result);
    }

    #[test]
    fn test_0829_example_2() {
        let n = 9;
        let result = 3;

        assert_eq!(Solution::consecutive_numbers_sum(n), result);
    }

    #[test]
    fn test_0829_example_3() {
        let n = 15;
        let result = 4;

        assert_eq!(Solution::consecutive_numbers_sum(n), result);
    }
}
