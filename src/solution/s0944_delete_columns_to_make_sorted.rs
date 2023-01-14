/**
 * [0944] Delete Columns to Make Sorted
 *
 * You are given an array of n strings strs, all of the same length.
 * The strings can be arranged such that there is one on each line, making a grid.
 *
 * 	For example, strs = ["abc", "bce", "cae"] can be arranged as follows:
 *
 * abc
 * bce
 * cae
 *
 * You want to delete the columns that are not sorted lexicographically. In the above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e') are sorted, while column 1 ('b', 'c', 'a') is not, so you would delete column 1.
 * Return the number of columns that you will delete.
 *  
 * Example 1:
 *
 * Input: strs = ["cba","daf","ghi"]
 * Output: 1
 * Explanation: The grid looks as follows:
 *   cba
 *   daf
 *   ghi
 * Columns 0 and 2 are sorted, but column 1 is not, so you only need to delete 1 column.
 *
 * Example 2:
 *
 * Input: strs = ["a","b"]
 * Output: 0
 * Explanation: The grid looks as follows:
 *   a
 *   b
 * Column 0 is the only column and is sorted, so you will not delete any columns.
 *
 * Example 3:
 *
 * Input: strs = ["zyx","wvu","tsr"]
 * Output: 3
 * Explanation: The grid looks as follows:
 *   zyx
 *   wvu
 *   tsr
 * All 3 columns are not sorted, so you will delete all 3.
 *
 *  
 * Constraints:
 *
 * 	n == strs.length
 * 	1 <= n <= 100
 * 	1 <= strs[i].length <= 1000
 * 	strs[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-columns-to-make-sorted/
// discuss: https://leetcode.com/problems/delete-columns-to-make-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let cols = strs[0].len();
        let strs: Vec<&[u8]> = strs.iter().map(String::as_bytes).collect();
        (0..cols).fold(0, |removed, col| {
            removed + strs.windows(2).any(|w| w[0][col] > w[1][col]) as i32
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0944_example_1() {
        let strs = vec_string!["cba", "daf", "ghi"];
        let result = 1;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }

    #[test]
    fn test_0944_example_2() {
        let strs = vec_string!["a", "b"];
        let result = 0;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }

    #[test]
    fn test_0944_example_3() {
        let strs = vec_string!["zyx", "wvu", "tsr"];
        let result = 3;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }
}
