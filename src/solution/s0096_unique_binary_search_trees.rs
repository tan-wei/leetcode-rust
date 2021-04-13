/**
 * [96] Unique Binary Search Trees
 *
 * Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: 5
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 19
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-binary-search-trees/
// discuss: https://leetcode.com/problems/unique-binary-search-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut result = vec![1];
        let n = n as usize;
        for i in 1..=n {
            let mut t = 0;
            for j in 0..i {
                t += result[j] * result[i - j - 1];
            }
            result.push(t);
        }

        *result.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0096_example_1() {
        let n = 3;
        let result = 5;

        assert_eq!(Solution::num_trees(n), result);
    }

    #[test]
    fn test_0096_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::num_trees(n), result);
    }
}
