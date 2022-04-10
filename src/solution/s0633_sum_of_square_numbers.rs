/**
 * [0633] Sum of Square Numbers
 *
 * Given a non-negative integer c, decide whether there're two integers a and b such that a^2 + b^2 = c.
 *  
 * Example 1:
 *
 * Input: c = 5
 * Output: true
 * Explanation: 1 * 1 + 2 * 2 = 5
 *
 * Example 2:
 *
 * Input: c = 3
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	0 <= c <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-square-numbers/
// discuss: https://leetcode.com/problems/sum-of-square-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let (mut left, mut right) = (0u64, (c as f64).sqrt() as u64);

        while left <= right {
            let n = left.pow(2) + right.pow(2);
            match n.cmp(&(c as u64)) {
                std::cmp::Ordering::Less => {
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    right -= 1;
                }
                std::cmp::Ordering::Equal => return true,
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
    fn test_0633_example_1() {
        let c = 5;
        let result = true;

        assert_eq!(Solution::judge_square_sum(c), result);
    }

    #[test]
    fn test_0633_example_2() {
        let c = 3;
        let result = false;

        assert_eq!(Solution::judge_square_sum(c), result);
    }

    #[test]
    fn test_0633_additional_1() {
        let c = 2147483600;
        let result = true;

        assert_eq!(Solution::judge_square_sum(c), result);
    }
}
