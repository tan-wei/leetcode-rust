/**
 * [2193] Minimum Number of Moves to Make Palindrome
 *
 * You are given a string s consisting only of lowercase English letters.
 * In one move, you can select any two adjacent characters of s and swap them.
 * Return the minimum number of moves needed to make s a palindrome.
 * Note that the input will be generated such that s can always be converted to a palindrome.
 *  
 * Example 1:
 *
 * Input: s = "aabb"
 * Output: 2
 * Explanation:
 * We can obtain two palindromes from s, "abba" and "baab".
 * - We can obtain "abba" from s in 2 moves: "a<u>ab</u>b" -> "ab<u>ab</u>" -> "abba".
 * - We can obtain "baab" from s in 2 moves: "a<u>ab</u>b" -> "<u>ab</u>ab" -> "baab".
 * Thus, the minimum number of moves needed to make s a palindrome is 2.
 *
 * Example 2:
 *
 * Input: s = "letelt"
 * Output: 2
 * Explanation:
 * One of the palindromes we can obtain from s in 2 moves is "lettel".
 * One of the ways we can obtain it is "lete<u>lt</u>" -> "let<u>et</u>l" -> "lettel".
 * Other palindromes such as "tleelt" can also be obtained in 2 moves.
 * It can be shown that it is not possible to obtain a palindrome in less than 2 moves.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists only of lowercase English letters.
 * 	s can be converted to a palindrome using a finite number of moves.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/
// discuss: https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2193_example_1() {
        let s = "aabb".to_string();

        let result = 2;

        assert_eq!(Solution::min_moves_to_make_palindrome(s), result);
    }

    #[test]
    #[ignore]
    fn test_2193_example_2() {
        let s = "letelt".to_string();

        let result = 2;

        assert_eq!(Solution::min_moves_to_make_palindrome(s), result);
    }
}
