/**
 * [0726] Number of Atoms
 *
 * Given a string formula representing a chemical formula, return the count of each atom.
 * The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.
 * One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow.
 *
 * 	For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.
 *
 * Two formulas are concatenated together to produce another formula.
 *
 * 	For example, "H2O2He3Mg4" is also a formula.
 *
 * A formula placed in parentheses, and a count (optionally added) is also a formula.
 *
 * 	For example, "(H2O2)" and "(H2O2)3" are formulas.
 *
 * Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1), followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.
 * The test cases are generated so that all the values in the output fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: formula = "H2O"
 * Output: "H2O"
 * Explanation: The count of elements are {'H': 2, 'O': 1}.
 *
 * Example 2:
 *
 * Input: formula = "Mg(OH)2"
 * Output: "H2MgO2"
 * Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.
 *
 * Example 3:
 *
 * Input: formula = "K4(ON(SO3)2)2"
 * Output: "K4N2O14S4"
 * Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
 *
 *  
 * Constraints:
 *
 * 	1 <= formula.length <= 1000
 * 	formula consists of English letters, digits, '(', and ')'.
 * 	formula is always valid.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-atoms/
// discuss: https://leetcode.com/problems/number-of-atoms/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-atoms/discuss/214835/0ms-parser-in-Rust.
    pub fn count_of_atoms(formula: String) -> String {
        let mut counts = std::collections::HashMap::new();
        let mut stack: Vec<std::collections::HashMap<&str, u32>> = Vec::new();
        let mut formula: &str = &formula;
        while !formula.is_empty() {
            if formula.starts_with("(") {
                formula = formula.get(1..).unwrap(); // consume (
                stack.push(counts);
                counts = std::collections::HashMap::new();
            } else if formula.starts_with(")") {
                formula = formula.get(1..).unwrap(); // consume )
                let count_len = formula.chars().take_while(|c| c.is_digit(10)).count();
                let count = if count_len > 0 {
                    formula[..count_len].parse().unwrap()
                } else {
                    1
                };
                formula = formula.get(count_len..).unwrap(); // consume count
                for (_, val) in counts.iter_mut() {
                    *val *= count;
                }
                counts = match stack.pop() {
                    Some(mut top) => {
                        for (key, val) in counts.iter() {
                            *top.entry(key).or_insert(0) += *val;
                        }
                        top
                    }
                    None => counts,
                };
            } else {
                // capital letter followed by lowercase letters
                let name_len = 1 + formula
                    .chars()
                    .skip(1)
                    .take_while(|c| c.is_lowercase())
                    .count();
                let name = &formula[..name_len];
                formula = formula.get(name_len..).unwrap(); // consume name
                let count_len = formula.chars().take_while(|c| c.is_digit(10)).count();
                let count = if count_len > 0 {
                    formula[..count_len].parse().unwrap()
                } else {
                    1
                };
                formula = formula.get(count_len..).unwrap(); // consume count
                *counts.entry(name).or_insert(0) += count;
            }
        }
        let mut elements: Vec<_> = counts.iter().collect();
        elements.sort_unstable();
        let mut result = String::new();
        for (name, count) in elements {
            result.push_str(&name);
            if *count > 1 {
                result.push_str(&count.to_string());
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0726_example_1() {
        let formula = "H2O".to_string();
        let result = "H2O".to_string();

        assert_eq!(Solution::count_of_atoms(formula), result);
    }

    #[test]
    fn test_0726_example_2() {
        let formula = "Mg(OH)2".to_string();
        let result = "H2MgO2".to_string();

        assert_eq!(Solution::count_of_atoms(formula), result);
    }

    #[test]
    fn test_0726_example_3() {
        let formula = "K4(ON(SO3)2)2".to_string();
        let result = "K4N2O14S4".to_string();

        assert_eq!(Solution::count_of_atoms(formula), result);
    }
}
