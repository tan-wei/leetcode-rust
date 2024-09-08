/**
 * [1678] Goal Parser Interpretation
 *
 * You own a Goal Parser that can interpret a string command. The command consists of an alphabet of "G", "()" and/or "(al)" in some order. The Goal Parser will interpret "G" as the string "G", "()" as the string "o", and "(al)" as the string "al". The interpreted strings are then concatenated in the original order.
 * Given the string command, return the Goal Parser's interpretation of command.
 *  
 * Example 1:
 *
 * Input: command = "G()(al)"
 * Output: "Goal"
 * Explanation: The Goal Parser interprets the command as follows:
 * G -> G
 * () -> o
 * (al) -> al
 * The final concatenated result is "Goal".
 *
 * Example 2:
 *
 * Input: command = "G()()()()(al)"
 * Output: "Gooooal"
 *
 * Example 3:
 *
 * Input: command = "(al)G(al)()()G"
 * Output: "alGalooG"
 *
 *  
 * Constraints:
 *
 * 	1 <= command.length <= 100
 * 	command consists of "G", "()", and/or "(al)" in some order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/goal-parser-interpretation/
// discuss: https://leetcode.com/problems/goal-parser-interpretation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn interpret(command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1678_example_1() {
        let command = "G()(al)".to_string();

        let result = "Goal".to_string();

        assert_eq!(Solution::interpret(command), result);
    }

    #[test]
    fn test_1678_example_2() {
        let command = "G()()()()(al)".to_string();

        let result = "Gooooal".to_string();

        assert_eq!(Solution::interpret(command), result);
    }

    #[test]
    fn test_1678_example_3() {
        let command = "(al)G(al)()()G".to_string();

        let result = "alGalooG".to_string();

        assert_eq!(Solution::interpret(command), result);
    }
}
