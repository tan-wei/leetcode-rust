/**
 * [2169] Count Operations to Obtain Zero
 *
 * You are given two non-negative integers num1 and num2.
 * In one operation, if num1 >= num2, you must subtract num2 from num1, otherwise subtract num1 from num2.
 *
 * 	For example, if num1 = 5 and num2 = 4, subtract num2 from num1, thus obtaining num1 = 1 and num2 = 4. However, if num1 = 4 and num2 = 5, after one operation, num1 = 4 and num2 = 1.
 *
 * Return the number of operations required to make either num1 = 0 or num2 = 0.
 *  
 * Example 1:
 *
 * Input: num1 = 2, num2 = 3
 * Output: 3
 * Explanation:
 * - Operation 1: num1 = 2, num2 = 3. Since num1 < num2, we subtract num1 from num2 and get num1 = 2, num2 = 3 - 2 = 1.
 * - Operation 2: num1 = 2, num2 = 1. Since num1 > num2, we subtract num2 from num1.
 * - Operation 3: num1 = 1, num2 = 1. Since num1 == num2, we subtract num2 from num1.
 * Now num1 = 0 and num2 = 1. Since num1 == 0, we do not need to perform any further operations.
 * So the total number of operations required is 3.
 *
 * Example 2:
 *
 * Input: num1 = 10, num2 = 10
 * Output: 1
 * Explanation:
 * - Operation 1: num1 = 10, num2 = 10. Since num1 == num2, we subtract num2 from num1 and get num1 = 10 - 10 = 0.
 * Now num1 = 0 and num2 = 10. Since num1 == 0, we are done.
 * So the total number of operations required is 1.
 *
 *  
 * Constraints:
 *
 * 	0 <= num1, num2 <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-operations-to-obtain-zero/
// discuss: https://leetcode.com/problems/count-operations-to-obtain-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        if num1 == 0 || num2 == 0 {
            return 0;
        }

        if num1 > num2 {
            return 1 + Solution::count_operations(num1 - num2, num2);
        }

        if num2 > num1 {
            return 1 + Solution::count_operations(num1, num2 - num1);
        }

        if num1 == num2 {
            return 1;
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2169_example_1() {
        let num1 = 2;
        let num2 = 3;

        let result = 3;

        assert_eq!(Solution::count_operations(num1, num2), result);
    }

    #[test]
    fn test_2169_example_2() {
        let num1 = 10;
        let num2 = 10;

        let result = 1;

        assert_eq!(Solution::count_operations(num1, num2), result);
    }
}
