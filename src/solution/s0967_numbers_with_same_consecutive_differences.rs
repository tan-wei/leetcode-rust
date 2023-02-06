/**
 * [0967] Numbers With Same Consecutive Differences
 *
 * Given two integers n and k, return an array of all the integers of length n where the difference between every two consecutive digits is k. You may return the answer in any order.
 * Note that the integers should not have leading zeros. Integers as 02 and 043 are not allowed.
 *  
 * Example 1:
 *
 * Input: n = 3, k = 7
 * Output: [181,292,707,818,929]
 * Explanation: Note that 070 is not a valid number, because it has leading zeroes.
 *
 * Example 2:
 *
 * Input: n = 2, k = 1
 * Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 9
 * 	0 <= k <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/numbers-with-same-consecutive-differences/
// discuss: https://leetcode.com/problems/numbers-with-same-consecutive-differences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/numbers-with-same-consecutive-differences/solutions/798476/rust-bfs-solution/
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = (1..=9).collect();
        for _ in 0..n - 1 {
            let mut v = vec![];
            for m in result.iter() {
                for d in if k == 0 { vec![0] } else { vec![-k, k] }.iter() {
                    let j = (m % 10) + d;
                    if j >= 0 && j < 10 {
                        v.push(m * 10 + j);
                    }
                }
            }
            result = v;
        }
        if n == 1 {
            result.push(0);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0967_example_1() {
        let n = 3;
        let k = 7;
        let result = vec![181, 292, 707, 818, 929];

        assert_eq!(Solution::nums_same_consec_diff(n, k), result);
    }

    #[test]
    fn test_0967_example_2() {
        let n = 2;
        let k = 1;
        let result = vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
        ];

        assert_eq!(Solution::nums_same_consec_diff(n, k), result);
    }
}
