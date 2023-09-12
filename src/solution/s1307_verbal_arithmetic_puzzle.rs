/**
 * [1307] Verbal Arithmetic Puzzle
 *
 * Given an equation, represented by words on the left side and the result on the right side.
 * You need to check if the equation is solvable under the following rules:
 *
 * 	Each character is decoded as one digit (0 - 9).
 * 	No two characters can map to the same digit.
 * 	Each words[i] and result are decoded as one number without leading zeros.
 * 	Sum of numbers on the left side (words) will equal to the number on the right side (result).
 *
 * Return true if the equation is solvable, otherwise return false.
 *
 * Example 1:
 *
 * Input: words = ["SEND","MORE"], result = "MONEY"
 * Output: true
 * Explanation: Map 'S'-> 9, 'E'->5, 'N'->6, 'D'->7, 'M'->1, 'O'->0, 'R'->8, 'Y'->'2'
 * Such that: "SEND" + "MORE" = "MONEY" ,  9567 + 1085 = 10652
 * Example 2:
 *
 * Input: words = ["SIX","SEVEN","SEVEN"], result = "TWENTY"
 * Output: true
 * Explanation: Map 'S'-> 6, 'I'->5, 'X'->0, 'E'->8, 'V'->7, 'N'->2, 'T'->1, 'W'->'3', 'Y'->4
 * Such that: "SIX" + "SEVEN" + "SEVEN" = "TWENTY" ,  650 + 68782 + 68782 = 138214
 * Example 3:
 *
 * Input: words = ["LEET","CODE"], result = "POINT"
 * Output: false
 * Explanation: There is no possible mapping to satisfy the equation, so we return false.
 * Note that two different characters cannot map to the same digit.
 *
 *
 * Constraints:
 *
 * 	2 <= words.length <= 5
 * 	1 <= words[i].length, result.length <= 7
 * 	words[i], result contain only uppercase English letters.
 * 	The number of different characters used in the expression is at most 10.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/verbal-arithmetic-puzzle/
// discuss: https://leetcode.com/problems/verbal-arithmetic-puzzle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::iter::FromIterator;

impl Solution {
    // Credit: https://leetcode.com/problems/verbal-arithmetic-puzzle/solutions/2088909/faster-than-100-of-rust-solutions-best-explanation/
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut equation: Vec<Vec<char>> = Vec::new();
        for w in words {
            if w.len() > result.len() {
                return false;
            }
            equation.push(w.chars().rev().collect());
        }
        let solution = &mut std::collections::HashMap::new();
        let ans: Vec<char> = result.chars().rev().collect();
        if Self::can_solve(&equation, &ans, 0, 0, 0, solution) {
            true
        } else {
            false
        }
    }

    fn can_solve(
        equation: &[Vec<char>],
        result: &[char],
        row: usize,
        col: usize,
        carry: u32,
        solution: &mut std::collections::HashMap<char, u8>,
    ) -> bool {
        let addend = row < equation.len();
        if addend && col >= equation[row].len() {
            // No leading zero for multicharacter words
            let word = &equation[row];
            if solution[&word[word.len() - 1]] == 0 && word.len() > 1 {
                return false;
            }
            return Self::can_solve(equation, result, row + 1, col, carry, solution);
        }
        if col == result.len() && !addend {
            return carry == 0 && (col == 1 || solution[&result[col - 1]] > 0);
        }

        let digit = if addend {
            equation[row][col]
        } else {
            result[col]
        };
        let assigned = solution.contains_key(&digit);

        if addend {
            if assigned {
                Self::can_solve(
                    equation,
                    result,
                    row + 1,
                    col,
                    carry + (solution[&digit] as u32),
                    solution,
                )
            } else {
                let used: std::collections::HashSet<&u8> =
                    std::collections::HashSet::from_iter(solution.values());
                let unused: Vec<u8> = (0..=9).filter(|x| !used.contains(x)).collect();
                for i in unused {
                    solution.insert(digit, i);
                    if Self::can_solve(equation, result, row + 1, col, carry + (i as u32), solution)
                    {
                        return true;
                    }
                    solution.remove(&digit);
                }
                false
            }
        } else {
            let sum_digit = (carry % 10) as u8;
            if assigned {
                (solution[&digit] == sum_digit)
                    && Self::can_solve(equation, result, 0, col + 1, carry / 10, solution)
            } else {
                let used = solution.values().any(|&x| x == sum_digit);
                if used {
                    return false;
                }
                solution.insert(digit, sum_digit);
                if Self::can_solve(equation, result, 0, col + 1, carry / 10, solution) {
                    return true;
                }
                solution.remove(&digit);
                false
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1307_example_1() {
        let words = vec_string!["SEND", "MORE"];
        let result = "MONEY".to_string();
        let res = true;

        assert_eq!(Solution::is_solvable(words, result), res);
    }

    #[test]
    fn test_1307_example_2() {
        let words = vec_string!["SIX", "SEVEN", "SEVEN"];
        let result = "TWENTY".to_string();
        let res = true;

        assert_eq!(Solution::is_solvable(words, result), res);
    }

    #[test]
    fn test_1307_example_3() {
        let words = vec_string!["LEET", "CODE"];
        let result = "POINT".to_string();
        let res = false;

        assert_eq!(Solution::is_solvable(words, result), res);
    }
}
