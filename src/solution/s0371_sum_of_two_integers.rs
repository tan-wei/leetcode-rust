/**
 * [371] Sum of Two Integers
 *
 * Given two integers a and b, return the sum of the two integers without using the operators + and -.
 *  
 * Example 1:
 * Input: a = 1, b = 2
 * Output: 3
 * Example 2:
 * Input: a = 2, b = 3
 * Output: 5
 *  
 * Constraints:
 *
 * 	-1000 <= a, b <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-two-integers/
// discuss: https://leetcode.com/problems/sum-of-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        fn sum_helper(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                sum_helper(a ^ b, (a & b) << 1)
            }
        }
        sum_helper(a, b)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0371_example_1() {
        let a = 1;
        let b = 2;

        assert_eq!(Solution::get_sum(a, b), a + b);
    }

    #[test]
    fn test_0371_example_2() {
        let a = 2;
        let b = 3;

        assert_eq!(Solution::get_sum(a, b), a + b);
    }
}
