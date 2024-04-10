/**
 * [1556] Thousand Separator
 *
 * Given an integer n, add a dot (".") as the thousands separator and return it in string format.
 *  
 * Example 1:
 *
 * Input: n = 987
 * Output: "987"
 *
 * Example 2:
 *
 * Input: n = 1234
 * Output: "1.234"
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/thousand-separator/
// discuss: https://leetcode.com/problems/thousand-separator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        n.to_string()
            .chars()
            .collect::<Vec<_>>()
            .rchunks(3)
            .rev()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(".")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1556_example_1() {
        let n = 987;

        let result = "987".to_string();

        assert_eq!(Solution::thousand_separator(n), result);
    }

    #[test]
    fn test_1556_example_2() {
        let n = 1234;

        let result = "1.234".to_string();

        assert_eq!(Solution::thousand_separator(n), result);
    }
}
