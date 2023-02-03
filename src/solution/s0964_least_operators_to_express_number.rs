/**
 * [0964] Least Operators to Express Number
 *
 * Given a single positive integer x, we will write an expression of the form x (op1) x (op2) x (op3) x ... where each operator op1, op2, etc. is either addition, subtraction, multiplication, or division (+, -, *, or /). For example, with x = 3, we might write 3 * 3 / 3 + 3 - 3 which is a value of <font face="monospace">3</font>.
 * When writing such an expression, we adhere to the following conventions:
 *
 * 	The division operator (/) returns rational numbers.
 * 	There are no parentheses placed anywhere.
 * 	We use the usual order of operations: multiplication and division happen before addition and subtraction.
 * 	It is not allowed to use the unary negation operator (-). For example, "x - x" is a valid expression as it only uses subtraction, but "-x + x" is not because it uses negation.
 *
 * We would like to write an expression with the least number of operators such that the expression equals the given target. Return the least number of operators used.
 *  
 * Example 1:
 *
 * Input: x = 3, target = 19
 * Output: 5
 * Explanation: 3 * 3 + 3 * 3 + 3 / 3.
 * The expression contains 5 operations.
 *
 * Example 2:
 *
 * Input: x = 5, target = 501
 * Output: 8
 * Explanation: 5 * 5 * 5 * 5 - 5 * 5 * 5 + 5 / 5.
 * The expression contains 8 operations.
 *
 * Example 3:
 *
 * Input: x = 100, target = 100000000
 * Output: 3
 * Explanation: 100 * 100 * 100 * 100.
 * The expression contains 3 operations.
 *
 *  
 * Constraints:
 *
 * 	2 <= x <= 100
 * 	1 <= target <= 2 * 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/least-operators-to-express-number/
// discuss: https://leetcode.com/problems/least-operators-to-express-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        match x.cmp(&target) {
            std::cmp::Ordering::Greater => std::cmp::min(target * 2 - 1, (x - target) * 2),
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => {
                let mut sums = x as i64;
                let target = target as i64;
                let mut times = 0;
                while sums < target {
                    times += 1;
                    sums *= x as i64;
                }

                if sums == target {
                    return times;
                }

                let mut l = i32::MAX;
                let mut r = i32::MAX;
                if sums - target < target {
                    // -
                    l = Self::least_ops_express_target(x, (sums - target) as i32) + times;
                }
                // +
                r = Self::least_ops_express_target(x, (target - (sums / x as i64)) as i32) + times
                    - 1;
                std::cmp::min(l, r) + 1
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0964_example_1() {
        let x = 3;
        let target = 19;
        let result = 5;

        assert_eq!(Solution::least_ops_express_target(x, target), result);
    }

    #[test]
    fn test_0964_example_2() {
        let x = 5;
        let target = 501;
        let result = 8;

        assert_eq!(Solution::least_ops_express_target(x, target), result);
    }

    #[test]
    fn test_0964_example_3() {
        let x = 100;
        let target = 100000000;
        let result = 3;

        assert_eq!(Solution::least_ops_express_target(x, target), result);
    }

    #[test]
    fn test_0964_additional_1() {
        let x = 79;
        let target = 155800339;
        let result = 45;

        assert_eq!(Solution::least_ops_express_target(x, target), result);
    }
}
