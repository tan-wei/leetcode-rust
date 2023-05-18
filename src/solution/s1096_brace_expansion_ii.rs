/**
 * [1096] Brace Expansion II
 *
 * Under the grammar given below, strings can represent a set of lowercase words. Let R(expr) denote the set of words the expression represents.
 * The grammar can best be understood through simple examples:
 *
 * 	Single letters represent a singleton set containing that word.
 *
 * 		R("a") = {"a"}
 * 		R("w") = {"w"}
 *
 *
 * 	When we take a comma-delimited list of two or more expressions, we take the union of possibilities.
 *
 * 		R("{a,b,c}") = {"a","b","c"}
 * 		R("{{a,b},{b,c}}") = {"a","b","c"} (notice the final set only contains each word at most once)
 *
 *
 * 	When we concatenate two expressions, we take the set of possible concatenations between two words where the first word comes from the first expression and the second word comes from the second expression.
 *
 * 		R("{a,b}{c,d}") = {"ac","ad","bc","bd"}
 * 		R("a{b,c}{d,e}f{g,h}") = {"abdfg", "abdfh", "abefg", "abefh", "acdfg", "acdfh", "acefg", "acefh"}
 *
 *
 *
 * Formally, the three rules for our grammar:
 *
 * 	For every lowercase letter x, we have R(x) = {x}.
 * 	For expressions e1, e2, ... , ek with k >= 2, we have R({e1, e2, ...}) = R(e1) &cup; R(e2) &cup; ...
 * 	For expressions e1 and e2, we have R(e1 + e2) = {a + b for (a, b) in R(e1) &times; R(e2)}, where + denotes concatenation, and &times; denotes the cartesian product.
 *
 * Given an expression representing a set of words under the given grammar, return the sorted list of words that the expression represents.
 *  
 * Example 1:
 *
 * Input: expression = "{a,b}{c,{d,e}}"
 * Output: ["ac","ad","ae","bc","bd","be"]
 *
 * Example 2:
 *
 * Input: expression = "{{a,z},a{b,c},{ab,z}}"
 * Output: ["a","ab","ac","z"]
 * Explanation: Each distinct word is written only once in the final answer.
 *
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 60
 * 	expression[i] consists of '{', '}', ','or lowercase English letters.
 * 	The given expression represents a set of words based on the grammar given in the description.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/brace-expansion-ii/
// discuss: https://leetcode.com/problems/brace-expansion-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut stack = Vec::<String>::new();
        let mut set = std::collections::HashSet::<String>::new();

        stack.push(expression);
        Self::dfs_helper(&mut stack, &mut set, &mut result);
        result.sort();

        result
    }

    fn dfs_helper(
        stack: &mut Vec<String>,
        set: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) {
        while !stack.is_empty() {
            let mut str = stack.pop().unwrap();
            if str.find('{') == None {
                if !set.contains(&str) {
                    set.insert(str.clone());
                    result.push(str.clone());
                }
                continue;
            }
            let mut i = 0;
            let mut l = 0;
            let mut r = 0;
            while let Some(ch) = str[i..].chars().next() {
                if ch == '}' {
                    break;
                }
                i += 1;
                if ch == '{' {
                    l = i - 1;
                }
            }
            r = i;

            let args: Vec<&str> = str[l + 1..r].split(",").collect();
            for s in args {
                let mut ss = String::new();
                ss.push_str(&str[0..l]);
                ss.push_str(s);
                ss.push_str(&str[r + 1..]);
                stack.push(ss);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1096_example_1() {
        let expression = "{a,b}{c,{d,e}}".to_string();
        let result = vec_string!["ac", "ad", "ae", "bc", "bd", "be"];

        assert_eq!(Solution::brace_expansion_ii(expression), result);
    }

    #[test]
    fn test_1096_example_2() {
        let expression = "{{a,z},a{b,c},{ab,z}}".to_string();
        let result = vec_string!["a", "ab", "ac", "z"];

        assert_eq!(Solution::brace_expansion_ii(expression), result);
    }
}
