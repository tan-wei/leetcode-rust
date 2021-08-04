/**
 * [241] Different Ways to Add Parentheses
 *
 * Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
 *  
 * Example 1:
 *
 * Input: expression = "2-1-1"
 * Output: [0,2]
 * Explanation:
 * ((2-1)-1) = 0
 * (2-(1-1)) = 2
 *
 * Example 2:
 *
 * Input: expression = "2*3-4*5"
 * Output: [-34,-14,-10,-10,10]
 * Explanation:
 * (2*(3-(4*5))) = -34
 * ((2*3)-(4*5)) = -14
 * ((2*(3-4))*5) = -10
 * (2*((3-4)*5)) = -10
 * (((2*3)-4)*5) = 10
 *
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 20
 * 	expression consists of digits and the operator '+', '-', and '*'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/different-ways-to-add-parentheses/
// discuss: https://leetcode.com/problems/different-ways-to-add-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result = vec![];
        for (i, c) in (0..).zip(expression.chars()) {
            if c == '+' || c == '-' || c == '*' {
                let lhs = Solution::diff_ways_to_compute((&expression[0..i]).to_string());
                let rhs = Solution::diff_ways_to_compute((&expression[i + 1..]).to_string());

                for l in lhs.iter() {
                    for r in rhs.iter() {
                        match c {
                            '+' => result.push(l + r),
                            '-' => result.push(l - r),
                            '*' => result.push(l * r),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        if result.is_empty() {
            result.push(expression.parse::<i32>().unwrap());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0241_example_1() {
        let expression = "2-1-1".to_string();
        let result = vec![0, 2];

        assert_eq_sorted!(Solution::diff_ways_to_compute(expression), result);
    }

    #[test]
    fn test_0241_example_2() {
        let expression = "2*3-4*5".to_string();
        let result = vec![-34, -14, -10, -10, 10];

        assert_eq_sorted!(Solution::diff_ways_to_compute(expression), result);
    }
}
