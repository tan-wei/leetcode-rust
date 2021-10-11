/**
 * [372] Super Pow
 *
 * Your task is to calculate a^b mod 1337 where a is a positive integer and b is an extremely large positive integer given in the form of an array.
 *  
 * Example 1:
 * Input: a = 2, b = [3]
 * Output: 8
 * Example 2:
 * Input: a = 2, b = [1,0]
 * Output: 1024
 * Example 3:
 * Input: a = 1, b = [4,3,3,8,5,2]
 * Output: 1
 * Example 4:
 * Input: a = 2147483647, b = [2,0,0]
 * Output: 1198
 *  
 * Constraints:
 *
 * 	1 <= a <= 2^31 - 1
 * 	1 <= b.length <= 2000
 * 	0 <= b[i] <= 9
 * 	b doesn't contain leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-pow/
// discuss: https://leetcode.com/problems/super-pow/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let a = a % 1337;
        let mut b = b;
        let mut result = 1;

        let tmp = match b.pop() {
            Some(n) => {
                for _ in 0..n {
                    result *= a;
                    result %= 1337;
                }
                Self::super_pow(a, b)
            }
            None => 1,
        };

        for _ in 0..10 {
            result *= tmp;
            result %= 1337;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0372_example_1() {
        let a = 2;
        let b = vec![3];
        let result = 8;

        assert_eq!(Solution::super_pow(a, b), result);
    }

    #[test]
    fn test_0372_example_2() {
        let a = 2;
        let b = vec![1, 0];
        let result = 1024;

        assert_eq!(Solution::super_pow(a, b), result);
    }

    #[test]
    fn test_0372_example_3() {
        let a = 1;
        let b = vec![4, 3, 3, 8, 5, 2];
        let result = 1;

        assert_eq!(Solution::super_pow(a, b), result);
    }

    #[test]
    fn test_0372_example_4() {
        let a = 2147483647;
        let b = vec![2, 0, 0];
        let result = 1198;

        assert_eq!(Solution::super_pow(a, b), result);
    }
}
