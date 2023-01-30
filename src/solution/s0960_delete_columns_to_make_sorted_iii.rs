/**
 * [0960] Delete Columns to Make Sorted III
 *
 * You are given an array of n strings strs, all of the same length.
 * We may choose any deletion indices, and we delete all the characters in those indices for each string.
 * For example, if we have strs = ["abcdef","uvwxyz"] and deletion indices {0, 2, 3}, then the final array after deletions is ["bef", "vyz"].
 * Suppose we chose a set of deletion indices answer such that after deletions, the final array has every string (row) in lexicographic order. (i.e., (strs[0][0] <= strs[0][1] <= ... <= strs[0][strs[0].length - 1]), and (strs[1][0] <= strs[1][1] <= ... <= strs[1][strs[1].length - 1]), and so on). Return the minimum possible value of answer.length.
 *  
 * Example 1:
 *
 * Input: strs = ["babca","bbazb"]
 * Output: 3
 * Explanation: After deleting columns 0, 1, and 4, the final array is strs = ["bc", "az"].
 * Both these rows are individually in lexicographic order (ie. strs[0][0] <= strs[0][1] and strs[1][0] <= strs[1][1]).
 * Note that strs[0] > strs[1] - the array strs is not necessarily in lexicographic order.
 * Example 2:
 *
 * Input: strs = ["edcba"]
 * Output: 4
 * Explanation: If we delete less than 4 columns, the only row will not be lexicographically sorted.
 *
 * Example 3:
 *
 * Input: strs = ["ghi","def","abc"]
 * Output: 0
 * Explanation: All rows are already lexicographically sorted.
 *
 *  
 * Constraints:
 *
 * 	n == strs.length
 * 	1 <= n <= 100
 * 	1 <= strs[i].length <= 100
 * 	strs[i] consists of lowercase English letters.
 *
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-columns-to-make-sorted-iii/
// discuss: https://leetcode.com/problems/delete-columns-to-make-sorted-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/delete-columns-to-make-sorted-iii/solutions/955785/rust-100/
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let len = strs[0].len();
        let mut dp: Vec<i32> = vec![1; len];
        let mut result = 1;
        for i in 0..len {
            for j in 0..i {
                let mut flag: bool = false;
                for item in strs.iter() {
                    let item = item.chars().collect::<Vec<char>>();
                    if item[i] >= item[j] {
                        flag = true;
                        continue;
                    }
                    flag = false;
                    break;
                }
                if flag {
                    dp[i] = std::cmp::max(dp[i], 1 + dp[j]);
                    result = std::cmp::max(result, dp[i]);
                }
            }
        }

        len as i32 - result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0960_example_1() {
        let strs = vec_string!["babca", "bbazb"];
        let result = 3;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }

    #[test]
    fn test_0960_example_2() {
        let strs = vec_string!["edcba"];
        let result = 4;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }

    #[test]
    fn test_0960_example_3() {
        let strs = vec_string!["ghi", "def", "abc"];
        let result = 0;

        assert_eq!(Solution::min_deletion_size(strs), result);
    }
}
