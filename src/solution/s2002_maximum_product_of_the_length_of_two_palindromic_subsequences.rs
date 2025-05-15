/**
 * [2002] Maximum Product of the Length of Two Palindromic Subsequences
 *
 * Given a string s, find two disjoint palindromic subsequences of s such that the product of their lengths is maximized. The two subsequences are disjoint if they do not both pick a character at the same index.
 * Return the maximum possible product of the lengths of the two palindromic subsequences.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters. A string is palindromic if it reads the same forward and backward.
 *  
 * Example 1:
 * <img alt="example-1" src="https://assets.leetcode.com/uploads/2021/08/24/two-palindromic-subsequences.png" style="width: 550px; height: 124px;" />
 * Input: s = "leetcodecom"
 * Output: 9
 * Explanation: An optimal solution is to choose "ete" for the 1^st subsequence and "cdc" for the 2^nd subsequence.
 * The product of their lengths is: 3 * 3 = 9.
 *
 * Example 2:
 *
 * Input: s = "bb"
 * Output: 1
 * Explanation: An optimal solution is to choose "b" (the first character) for the 1^st subsequence and "b" (the second character) for the 2^nd subsequence.
 * The product of their lengths is: 1 * 1 = 1.
 *
 * Example 3:
 *
 * Input: s = "accbcaxxcxx"
 * Output: 25
 * Explanation: An optimal solution is to choose "accca" for the 1^st subsequence and "xxcxx" for the 2^nd subsequence.
 * The product of their lengths is: 5 * 5 = 25.
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 12
 * 	s consists of lowercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/
// discuss: https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2002_example_1() {
        let s = "leetcodecom".to_string();

        let result = 9;

        assert_eq!(Solution::max_product(s), result);
    }

    #[test]
    #[ignore]
    fn test_2002_example_2() {
        let s = "bb".to_string();

        let result = 1;

        assert_eq!(Solution::max_product(s), result);
    }

    #[test]
    #[ignore]
    fn test_2002_example_3() {
        let s = "accbcaxxcxx".to_string();

        let result = 25;

        assert_eq!(Solution::max_product(s), result);
    }
}
