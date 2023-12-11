/**
 * [1414] Find the Minimum Number of Fibonacci Numbers Whose Sum Is K
 *
 * Given an integer k, return the minimum number of Fibonacci numbers whose sum is equal to k. The same Fibonacci number can be used multiple times.
 * The Fibonacci numbers are defined as:
 *
 * 	F1 = 1
 * 	F2 = 1
 * 	Fn = Fn-1 + Fn-2 for n > 2.
 * It is guaranteed that for the given constraints we can always find such Fibonacci numbers that sum up to k.
 *  
 * Example 1:
 *
 * Input: k = 7
 * Output: 2
 * Explanation: The Fibonacci numbers are: 1, 1, 2, 3, 5, 8, 13, ...
 * For k = 7 we can use 2 + 5 = 7.
 * Example 2:
 *
 * Input: k = 10
 * Output: 2
 * Explanation: For k = 10 we can use 2 + 8 = 10.
 *
 * Example 3:
 *
 * Input: k = 19
 * Output: 3
 * Explanation: For k = 19 we can use 1 + 5 + 13 = 19.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/
// discuss: https://leetcode.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibs = vec![1, 1];

        while fibs.last().unwrap() < &k {
            let n = fibs.len();
            fibs.push(fibs[n - 1] + fibs[n - 2]);
        }

        let mut k = k;
        let mut result = 0;

        while k > 0 {
            let n = fibs.len();
            if fibs[n - 1] <= k {
                k -= fibs[n - 1];
                result += 1;
            }
            fibs.pop();
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1414_example_1() {
        let k = 7;

        let result = 2;

        assert_eq!(Solution::find_min_fibonacci_numbers(k), result);
    }

    #[test]
    fn test_1414_example_2() {
        let k = 10;

        let result = 2;

        assert_eq!(Solution::find_min_fibonacci_numbers(k), result);
    }
}
