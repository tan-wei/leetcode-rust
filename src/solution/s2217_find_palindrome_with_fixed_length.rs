/**
 * [2217] Find Palindrome With Fixed Length
 *
 * Given an integer array queries and a positive integer intLength, return an array answer where answer[i] is either the queries[i]^th smallest positive palindrome of length intLength or -1 if no such palindrome exists.
 * A palindrome is a number that reads the same backwards and forwards. Palindromes cannot have leading zeros.
 *  
 * Example 1:
 *
 * Input: queries = [1,2,3,4,5,90], intLength = 3
 * Output: [101,111,121,131,141,999]
 * Explanation:
 * The first few palindromes of length 3 are:
 * 101, 111, 121, 131, 141, 151, 161, 171, 181, 191, 202, ...
 * The 90^th palindrome of length 3 is 999.
 *
 * Example 2:
 *
 * Input: queries = [2,4,6], intLength = 4
 * Output: [1111,1331,1551]
 * Explanation:
 * The first six palindromes of length 4 are:
 * 1001, 1111, 1221, 1331, 1441, and 1551.
 *
 *  
 * Constraints:
 *
 * 	1 <= queries.length <= 5 * 10^4
 * 	1 <= queries[i] <= 10^9
 * 	1 <= intLength <= 15
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-palindrome-with-fixed-length/
// discuss: https://leetcode.com/problems/find-palindrome-with-fixed-length/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2217_example_1() {
        let queries = vec![1, 2, 3, 4, 5, 90];
        let int_length = 3;

        let result = vec![101, 111, 121, 131, 141, 999];

        assert_eq!(Solution::kth_palindrome(queries, int_length), result);
    }

    #[test]
    #[ignore]
    fn test_2217_example_2() {
        let queries = vec![2, 4, 6];
        let int_length = 4;

        let result = vec![1111, 1331, 1551];

        assert_eq!(Solution::kth_palindrome(queries, int_length), result);
    }
}
