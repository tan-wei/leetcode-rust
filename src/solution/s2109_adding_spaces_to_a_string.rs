/**
 * [2109] Adding Spaces to a String
 *
 * You are given a 0-indexed string s and a 0-indexed integer array spaces that describes the indices in the original string where spaces will be added. Each space should be inserted before the character at the given index.
 *
 * 	For example, given s = "EnjoyYourCoffee" and spaces = [5, 9], we place spaces before 'Y' and 'C', which are at indices 5 and 9 respectively. Thus, we obtain "Enjoy <u>Y</u>our <u>C</u>offee".
 *
 * Return the modified string after the spaces have been added.
 *  
 * Example 1:
 *
 * Input: s = "LeetcodeHelpsMeLearn", spaces = [8,13,15]
 * Output: "Leetcode Helps Me Learn"
 * Explanation:
 * The indices 8, 13, and 15 correspond to the underlined characters in "Leetcode<u>H</u>elps<u>M</u>e<u>L</u>earn".
 * We then place spaces before those characters.
 *
 * Example 2:
 *
 * Input: s = "icodeinpython", spaces = [1,5,7,9]
 * Output: "i code in py thon"
 * Explanation:
 * The indices 1, 5, 7, and 9 correspond to the underlined characters in "i<u>c</u>ode<u>i</u>n<u>p</u>y<u>t</u>hon".
 * We then place spaces before those characters.
 *
 * Example 3:
 *
 * Input: s = "spacing", spaces = [0,1,2,3,4,5,6]
 * Output: " s p a c i n g"
 * Explanation:
 * We are also able to place spaces before the first character of the string.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists only of lowercase and uppercase English letters.
 * 	1 <= spaces.length <= 3 * 10^5
 * 	0 <= spaces[i] <= s.length - 1
 * 	All the values of spaces are strictly increasing.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/adding-spaces-to-a-string/
// discuss: https://leetcode.com/problems/adding-spaces-to-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2109_example_1() {
        let s = "LeetcodeHelpsMeLearn".to_string();
        let spaces = vec![8, 13, 15];

        let result = "Leetcode Helps Me Learn".to_string();

        assert_eq!(Solution::add_spaces(s, spaces), result);
    }

    #[test]
    #[ignore]
    fn test_2109_example_2() {
        let s = "icodeinpython".to_string();
        let spaces = vec![1, 5, 7, 9];

        let result = "i code in py thon".to_string();

        assert_eq!(Solution::add_spaces(s, spaces), result);
    }

    #[test]
    #[ignore]
    fn test_2109_example_3() {
        let s = "spacing".to_string();
        let spaces = vec![0, 1, 2, 3, 4, 5, 6];

        let result = " s p a c i n g".to_string();

        assert_eq!(Solution::add_spaces(s, spaces), result);
    }
}
