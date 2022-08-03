/**
 * [0779] K-th Symbol in Grammar
 *
 * We build a table of n rows (1-indexed). We start by writing 0 in the 1^st row. Now in every subsequent row, we look at the previous row and replace each occurrence of 0 with 01, and each occurrence of 1 with 10.
 *
 * 	For example, for n = 3, the 1^st row is 0, the 2^nd row is 01, and the 3^rd row is 0110.
 *
 * Given two integer n and k, return the k^th (1-indexed) symbol in the n^th row of a table of n rows.
 *  
 * Example 1:
 *
 * Input: n = 1, k = 1
 * Output: 0
 * Explanation: row 1: <u>0</u>
 *
 * Example 2:
 *
 * Input: n = 2, k = 1
 * Output: 0
 * Explanation:
 * row 1: 0
 * row 2: <u>0</u>1
 *
 * Example 3:
 *
 * Input: n = 2, k = 2
 * Output: 1
 * Explanation:
 * row 1: 0
 * row 2: 0<u>1</u>
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 30
 * 	1 <= k <= 2^n - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-symbol-in-grammar/
// discuss: https://leetcode.com/problems/k-th-symbol-in-grammar/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        // NOTE: Thue-Morse sequence
        (k - 1).count_ones() as i32 % 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0779_example_1() {
        let n = 1;
        let k = 1;
        let result = 0;

        assert_eq!(Solution::kth_grammar(n, k), result);
    }

    #[test]
    fn test_0779_example_2() {
        let n = 2;
        let k = 1;
        let result = 0;

        assert_eq!(Solution::kth_grammar(n, k), result);
    }

    #[test]
    fn test_0779_example_3() {
        let n = 2;
        let k = 2;
        let result = 1;

        assert_eq!(Solution::kth_grammar(n, k), result);
    }
}
