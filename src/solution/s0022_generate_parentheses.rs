/**
 * [22] Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *  
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *  
 * Constraints:
 *
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec![];
        }

        Solution::generate_parenthesis_recursive(n)
    }

    fn generate_parenthesis_recursive(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec!["".to_string()];
        }

        if n == 1 {
            return vec!["()".to_string()];
        }

        let mut result: Vec<String> = Vec::new();

        for i in 0..n {
            let result_i = Self::generate_parenthesis_recursive(i);
            let result_n_1_i = Self::generate_parenthesis_recursive(n - 1 - i);

            for left in result_i.iter() {
                for right in result_n_1_i.iter() {
                    result.push(format!("({}){}", left, right));
                }
            }
        }

        result
    }

    // Credit: https://leetcode.com/problems/generate-parentheses/discuss/950023/Rust-index-solution
    pub fn generate_parenthesis_v2(n: i32) -> Vec<String> {
        fn perm(positions: &mut Vec<usize>, p: usize, iteration: usize) -> Vec<Vec<usize>> {
            let mut combs = Vec::new();
            let saved_p = positions[p];
            positions[p] += iteration;
            for i in 0.. {
                if p < positions.len() - 1 {
                    combs.extend(perm(positions, p + 1, iteration + i));
                } else {
                    combs.push(positions.clone());
                }
                positions[p] += 1;
                if positions[p] > p * 2 {
                    break;
                }
            }
            positions[p] = saved_p;
            combs
        }

        if n <= 0 {
            return vec![];
        }

        if n == 1 {
            return vec!["()".into()];
        }

        perm(&mut (0..n as usize).collect(), 1, 0)
            .into_iter()
            .map(|r| {
                let mut iter = r.iter();
                (0..n as usize * 2)
                    .scan(iter.next(), |it, j| {
                        Some(if Some(&j) == *it {
                            *it = iter.next();
                            '('
                        } else {
                            ')'
                        })
                    })
                    .collect()
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0022_example_1() {
        assert_eq_sorted!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );

        assert_eq_sorted!(
            Solution::generate_parenthesis_v2(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn test_0022_example_2() {
        assert_eq_sorted!(Solution::generate_parenthesis(1), vec!["()"]);

        assert_eq_sorted!(Solution::generate_parenthesis_v2(1), vec!["()"]);
    }

    #[test]
    fn test_0022_non_positive_input() {
        assert_eq_sorted!(Solution::generate_parenthesis(0), Vec::<String>::new());

        assert_eq_sorted!(Solution::generate_parenthesis_v2(0), Vec::<String>::new());
    }
}
