/**
 * [0679] 24 Game
 *
 * You are given an integer array cards of length 4. You have four cards, each containing a number in the range [1, 9]. You should arrange the numbers on these cards in a mathematical expression using the operators ['+', '-', '*', '/'] and the parentheses '(' and ')' to get the value 24.
 * You are restricted with the following rules:
 *
 * 	The division operator '/' represents real division, not integer division.
 *
 * 		For example, 4 / (1 - 2 / 3) = 4 / (1 / 3) = 12.
 *
 *
 * 	Every operation done is between two numbers. In particular, we cannot use '-' as a unary operator.
 *
 * 		For example, if cards = [1, 1, 1, 1], the expression "-1 - 1 - 1 - 1" is not allowed.
 *
 *
 * 	You cannot concatenate numbers together
 *
 * 		For example, if cards = [1, 2, 1, 2], the expression "12 + 12" is not valid.
 *
 *
 *
 * Return true if you can get such expression that evaluates to 24, and false otherwise.
 *  
 * Example 1:
 *
 * Input: cards = [4,1,8,7]
 * Output: true
 * Explanation: (8-4) * (7-1) = 24
 *
 * Example 2:
 *
 * Input: cards = [1,2,1,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	cards.length == 4
 * 	1 <= cards[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/24-game/
// discuss: https://leetcode.com/problems/24-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let nums = nums.iter().map(|&x| x as f64).collect::<Vec<f64>>();

        const F: [fn(f64, f64) -> Option<f64>; 6] = [
            |a, b| Some(a + b),
            |a, b| Some(a - b),
            |a, b| Some(b - a),
            |a, b| Some(a * b),
            |a, b| if b != 0.0 { Some(a / b) } else { None },
            |a, b| if a != 0.0 { Some(b / a) } else { None },
        ];

        fn calc(nums: Vec<f64>) -> Vec<Vec<f64>> {
            let l = nums.len();

            if l <= 1 {
                return vec![nums];
            }

            let mut result = Vec::new();
            for i in 0..(l - 1) {
                for j in (i + 1)..l {
                    let n1 = nums[i];
                    let n2 = nums[j];
                    let nums = nums
                        .iter()
                        .enumerate()
                        .filter(|p| p.0 != i && p.0 != j)
                        .map(|p| *p.1)
                        .collect::<Vec<f64>>();

                    for f in F.iter() {
                        let mut nums = nums.clone();
                        if let Some(n) = f(n1, n2) {
                            nums.push(n);
                            result.push(nums);
                        }
                    }
                }
            }

            result
        }

        let mut stack = Vec::new();
        stack.push(nums);

        while let Some(top) = stack.pop() {
            if top.len() == 1 && (top[0] - 24.0f64).abs() <= f64::EPSILON * 100.0 {
                return true;
            }
            if top.len() > 1 {
                stack.append(&mut calc(top));
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
    fn test_0679_example_1() {
        let cards = vec![4, 1, 8, 7];
        let result = true;

        assert_eq!(Solution::judge_point24(cards), result);
    }

    #[test]
    fn test_0679_example_2() {
        let cards = vec![1, 2, 1, 2];
        let result = false;

        assert_eq!(Solution::judge_point24(cards), result);
    }

    #[test]
    fn test_0679_additional_1() {
        let cards = vec![3, 3, 8, 8];
        let result = true;

        assert_eq!(Solution::judge_point24(cards), result);
    }
}
