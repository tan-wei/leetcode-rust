/**
 * [0842] Split Array into Fibonacci Sequence
 *
 * You are given a string of digits num, such as "123456579". We can split it into a Fibonacci-like sequence [123, 456, 579].
 * Formally, a Fibonacci-like sequence is a list f of non-negative integers such that:
 *
 * 	0 <= f[i] < 2^31, (that is, each integer fits in a 32-bit signed integer type),
 * 	f.length >= 3, and
 * 	f[i] + f[i + 1] == f[i + 2] for all 0 <= i < f.length - 2.
 *
 * Note that when splitting the string into pieces, each piece must not have extra leading zeroes, except if the piece is the number 0 itself.
 * Return any Fibonacci-like sequence split from num, or return [] if it cannot be done.
 *  
 * Example 1:
 *
 * Input: num = "1101111"
 * Output: [11,0,11,11]
 * Explanation: The output [110, 1, 111] would also be accepted.
 *
 * Example 2:
 *
 * Input: num = "112358130"
 * Output: []
 * Explanation: The task is impossible.
 *
 * Example 3:
 *
 * Input: num = "0123"
 * Output: []
 * Explanation: Leading zeroes are not allowed, so "01", "2", "3" is not valid.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 200
 * 	num contains only digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-into-fibonacci-sequence/
// discuss: https://leetcode.com/problems/split-array-into-fibonacci-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut result = vec![];
        Self::dfs_helper(&num, &mut result, 0);
        result
    }

    fn dfs_helper(s: &str, result: &mut Vec<i32>, idx: usize) -> bool {
        if idx == s.len() && result.len() >= 3 {
            return true;
        }
        let mut num = 0i64;
        for i in idx..s.len() {
            if s.as_bytes()[idx] == b'0' && i > idx {
                break;
            }

            num = s[idx..i + 1].parse::<i64>().unwrap();
            if num > std::i32::MAX as i64 {
                break;
            }
            let size = result.len();
            if size >= 2 && num as i32 > result[size - 1] + result[size - 2] {
                break;
            }
            if size <= 1 || num as i32 == result[size - 1] + result[size - 2] {
                result.push(num as i32);
                if Self::dfs_helper(s, result, i + 1) {
                    return true;
                }
                result.pop();
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0842_example_1() {
        let num = "1101111".to_string();
        let result = vec![11, 0, 11, 11];

        assert_eq!(Solution::split_into_fibonacci(num), result);
    }

    #[test]
    fn test_0842_example_2() {
        let num = "112358130".to_string();
        let result = vec![];

        assert_eq!(Solution::split_into_fibonacci(num), result);
    }

    #[test]
    fn test_0842_example_3() {
        let num = "0123".to_string();
        let result = vec![];

        assert_eq!(Solution::split_into_fibonacci(num), result);
    }
}
