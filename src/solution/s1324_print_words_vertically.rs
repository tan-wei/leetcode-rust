/**
 * [1324] Print Words Vertically
 *
 * Given a string s. Return all the words vertically in the same order in which they appear in s.<br />
 * Words are returned as a list of strings, complete with spaces when is necessary. (Trailing spaces are not allowed).<br />
 * Each word would be put on only one column and that in one column there will be only one word.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "HOW ARE YOU"
 * Output: ["HAY","ORO","WEU"]
 * Explanation: Each word is printed vertically.
 *  "HAY"
 *  "ORO"
 *  "WEU"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "TO BE OR NOT TO BE"
 * Output: ["TBONTB","OEROOE","   T"]
 * Explanation: Trailing spaces is not allowed.
 * "TBONTB"
 * "OEROOE"
 * "   T"
 *
 *
 * Example 3:
 *
 *
 * Input: s = "CONTEST IS COMING"
 * Output: ["CIC","OSO","N M","T I","E N","S G","T"]
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= s.length <= 200
 * 	s contains only upper case English letters.
 * 	It's guaranteed that there is only one space between 2 words.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/print-words-vertically/
// discuss: https://leetcode.com/problems/print-words-vertically/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let s = s.trim();
        let s = s
            .split(" ")
            .into_iter()
            .map(|v| v.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut m = 0;
        let n = s.len();
        for i in 0..n {
            m = std::cmp::max(m, s[i].len());
        }

        let mut temps = vec![vec![' '; m]; n];
        for i in 0..n {
            for j in 0..s[i].len() {
                temps[i][j] = s[i][j];
            }
        }

        let mut result = vec![];
        for i in 0..m {
            let mut temp_str = String::from("");
            for j in 0..n {
                temp_str = format!("{}{}", temp_str, temps[j][i]);
            }
            result.push(temp_str.trim_end().to_string());
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1324_example_1() {
        let s = "HOW ARE YOU".to_string();
        let result = vec_string!["HAY", "ORO", "WEU"];

        assert_eq!(Solution::print_vertically(s), result);
    }

    #[test]
    fn test_1324_example_2() {
        let s = "TO BE OR NOT TO BE".to_string();
        let result = vec_string!["TBONTB", "OEROOE", "   T"];

        assert_eq!(Solution::print_vertically(s), result);
    }

    #[test]
    fn test_1324_example_3() {
        let s = "CONTEST IS COMING".to_string();
        let result = vec_string!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"];

        assert_eq!(Solution::print_vertically(s), result);
    }
}
