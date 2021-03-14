/**
 * [66] Plus One
 *
 * Given a non-empty array of decimal digits representing a non-negative integer, increment one to the integer.
 * The digits are stored such that the most significant digit is at the head of the list, and each element in the array contains a single digit.
 * You may assume the integer does not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 *
 * Input: digits = [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 *
 * Example 2:
 *
 * Input: digits = [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 *
 * Example 3:
 *
 * Input: digits = [0]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= digits.length <= 100
 * 	0 <= digits[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/plus-one/
// discuss: https://leetcode.com/problems/plus-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let n = digits.len();

        for i in (0..=n - 1).rev() {
            match digits[i] {
                9 => digits[i] = 0,
                a => {
                    digits[i] = a + 1;
                    return digits;
                }
            }
        }

        // become 1000...
        digits[0] = 1;
        digits.push(0);

        digits
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0066_example_1() {
        let digits = vec![1, 2, 3];
        let result = vec![1, 2, 4];

        assert_eq!(Solution::plus_one(digits), result);
    }

    #[test]
    fn test_0066_example_2() {
        let digits = vec![4, 3, 2, 1];
        let result = vec![4, 3, 2, 2];

        assert_eq!(Solution::plus_one(digits), result);
    }

    #[test]
    fn test_0066_example_3() {
        let digits = vec![0];
        let result = vec![1];

        assert_eq!(Solution::plus_one(digits), result);
    }
}
