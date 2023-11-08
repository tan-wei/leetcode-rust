/**
 * [1374] Generate a String With Characters That Have Odd Counts
 *
 * Given an integer n, return a string with n characters such that each character in such string occurs an odd number of times.
 * The returned string must contain only lowercase English letters. If there are multiples valid strings, return any of them.  
 *  
 * Example 1:
 *
 * Input: n = 4
 * Output: "pppz"
 * Explanation: "pppz" is a valid string since the character 'p' occurs three times and the character 'z' occurs once. Note that there are many other valid strings such as "ohhh" and "love".
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: "xy"
 * Explanation: "xy" is a valid string since the characters 'x' and 'y' occur once. Note that there are many other valid strings such as "ag" and "ur".
 *
 * Example 3:
 *
 * Input: n = 7
 * Output: "holasss"
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
// discuss: https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        std::iter::repeat('a')
            .take((n - 1 + n % 2) as usize)
            .chain(std::iter::once('b').take((1 - n % 2) as usize))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1374_example_1() {
        let n = 4;

        let result = "pppz".to_string();

        assert_eq!(Solution::generate_the_string(n), result);
    }

    #[test]
    #[ignore]
    fn test_1374_example_2() {
        let n = 2;

        let result = "xy".to_string();

        assert_eq!(Solution::generate_the_string(n), result);
    }

    #[test]
    #[ignore]
    fn test_1374_example_3() {
        let n = 7;

        let result = "holasss".to_string();

        assert_eq!(Solution::generate_the_string(n), result);
    }
}
