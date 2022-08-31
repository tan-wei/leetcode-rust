/**
 * [0808] Soup Servings
 *
 * There are two types of soup: type A and type B. Initially, we have n ml of each type of soup. There are four kinds of operations:
 * <ol>
 * 	Serve 100 ml of soup A and 0 ml of soup B,
 * 	Serve 75 ml of soup A and 25 ml of soup B,
 * 	Serve 50 ml of soup A and 50 ml of soup B, and
 * 	Serve 25 ml of soup A and 75 ml of soup B.
 * </ol>
 * When we serve some soup, we give it to someone, and we no longer have it. Each turn, we will choose from the four operations with an equal probability 0.25. If the remaining volume of soup is not enough to complete the operation, we will serve as much as possible. We stop once we no longer have some quantity of both types of soup.
 * Note that we do not have an operation where all 100 ml's of soup B are used first.
 * Return the probability that soup A will be empty first, plus half the probability that A and B become empty at the same time. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 *
 * Input: n = 50
 * Output: 0.62500
 * Explanation: If we choose the first two operations, A will become empty first.
 * For the third operation, A and B will become empty at the same time.
 * For the fourth operation, B will become empty first.
 * So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.25 * (1 + 1 + 0.5 + 0) = 0.625.
 *
 * Example 2:
 *
 * Input: n = 100
 * Output: 0.71875
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/soup-servings/
// discuss: https://leetcode.com/problems/soup-servings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let mut memo = vec![vec![0.0; 200]; 200];

        if n > 4800 {
            1.0
        } else {
            Self::helper((n + 24) / 25, (n + 24) / 25, &mut memo)
        }
    }

    fn helper(a: i32, b: i32, memo: &mut Vec<Vec<f64>>) -> f64 {
        if a <= 0 && b <= 0 {
            return 0.5;
        }

        if a <= 0 {
            return 1.0;
        }

        if b <= 0 {
            return 0.0;
        }

        if memo[a as usize][b as usize] > 0.0 {
            return memo[a as usize][b as usize];
        }

        memo[a as usize][b as usize] = 0.25
            * (Self::helper(a - 4, b, memo)
                + Self::helper(a - 3, b - 1, memo)
                + Self::helper(a - 2, b - 2, memo)
                + Self::helper(a - 1, b - 3, memo));
        memo[a as usize][b as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0808_example_1() {
        let n = 50;
        let result = 0.62500;

        assert_f64_near!(Solution::soup_servings(n), result);
    }

    #[test]
    fn test_0808_example_2() {
        let n = 100;
        let result = 0.71875;

        assert_f64_near!(Solution::soup_servings(n), result);
    }
}
