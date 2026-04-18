/**
 * [2434] Using a Robot to Print the Lexicographically Smallest String
 *
 * You are given a string s and a robot that currently holds an empty string t. Apply one of the following operations until s and t are both empty:
 *
 * 	Remove the first character of a string s and give it to the robot. The robot will append this character to the string t.
 * 	Remove the last character of a string t and give it to the robot. The robot will write this character on paper.
 *
 * Return the lexicographically smallest string that can be written on the paper.
 *  
 * Example 1:
 *
 * Input: s = "zza"
 * Output: "azz"
 * Explanation: Let p denote the written string.
 * Initially p="", s="zza", t="".
 * Perform first operation three times p="", s="", t="zza".
 * Perform second operation three times p="azz", s="", t="".
 *
 * Example 2:
 *
 * Input: s = "bac"
 * Output: "abc"
 * Explanation: Let p denote the written string.
 * Perform first operation twice p="", s="c", t="ba".
 * Perform second operation twice p="ab", s="c", t="".
 * Perform first operation p="ab", s="", t="c".
 * Perform second operation p="abc", s="", t="".
 *
 * Example 3:
 *
 * Input: s = "bdda"
 * Output: "addb"
 * Explanation: Let p denote the written string.
 * Initially p="", s="bdda", t="".
 * Perform first operation four times p="", s="", t="bdda".
 * Perform second operation four times p="addb", s="", t="".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only English lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
// discuss: https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2434_example_1() {
        let s = "zza".to_string();

        let result = "azz".to_string();

        assert_eq!(Solution::robot_with_string(s), result);
    }

    #[test]
    #[ignore]
    fn test_2434_example_2() {
        let s = "bac".to_string();

        let result = "abc".to_string();

        assert_eq!(Solution::robot_with_string(s), result);
    }

    #[test]
    #[ignore]
    fn test_2434_example_3() {
        let s = "bdda".to_string();

        let result = "addb".to_string();

        assert_eq!(Solution::robot_with_string(s), result);
    }
}
