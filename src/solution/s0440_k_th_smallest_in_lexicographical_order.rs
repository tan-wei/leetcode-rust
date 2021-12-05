/**
 * [0440] K-th Smallest in Lexicographical Order
 *
 * Given two integers n and k, return the k^th lexicographically smallest integer in the range [1, n].
 *  
 * Example 1:
 *
 * Input: n = 13, k = 2
 * Output: 10
 * Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.
 *
 * Example 2:
 *
 * Input: n = 1, k = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
// discuss: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut k = k as i64;
        let n = n as i64;
        if n < k || k < 1 {
            return 0;
        } else if n < 10 {
            return k as i32;
        }
        let mut result = 1;
        k -= 1;
        while k > 0 {
            let mut step = 0i64;
            let mut first = result;
            let mut last = result + 1;
            while first <= n {
                step += std::cmp::min((n + 1) as i64, last) - first;
                first *= 10;
                last *= 10;
            }

            if step <= k {
                result += 1;
                k -= step;
            } else {
                result *= 10;
                k -= 1;
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0440_example_1() {
        let n = 13;
        let k = 2;
        let result = 10;

        assert_eq!(Solution::find_kth_number(n, k), result);
    }

    #[test]
    fn test_0440_example_2() {
        let n = 1;
        let k = 1;
        let result = 1;

        assert_eq!(Solution::find_kth_number(n, k), result);
    }
}
