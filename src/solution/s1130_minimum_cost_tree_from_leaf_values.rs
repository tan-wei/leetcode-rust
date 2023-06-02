/**
 * [1130] Minimum Cost Tree From Leaf Values
 *
 * Given an array arr of positive integers, consider all binary trees such that:
 *
 * 	Each node has either 0 or 2 children;
 * 	The values of arr correspond to the values of each leaf in an in-order traversal of the tree.
 * 	The value of each non-leaf node is equal to the product of the largest leaf value in its left and right subtree, respectively.
 *
 * Among all possible binary trees considered, return the smallest possible sum of the values of each non-leaf node. It is guaranteed this sum fits into a 32-bit integer.
 * A node is a leaf if and only if it has zero children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/tree1.jpg" style="width: 500px; height: 169px;" />
 * Input: arr = [6,2,4]
 * Output: 32
 * Explanation: There are two possible trees shown.
 * The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/tree2.jpg" style="width: 224px; height: 145px;" />
 * Input: arr = [4,11]
 * Output: 44
 *
 *  
 * Constraints:
 *
 * 	2 <= arr.length <= 40
 * 	1 <= arr[i] <= 15
 * 	It is guaranteed that the answer fits into a 32-bit signed integer (i.e., it is less than 2^31).
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
// discuss: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/solutions/1047176/rust-0ms-2mb-memoized-recursive-dfs-need-help-with-time-complexity/
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashMap::new();
        Self::dfs_helper(&arr, 0, arr.len() - 1, &mut seen).1
    }

    fn dfs_helper(
        arr: &Vec<i32>,
        start: usize,
        end: usize,
        seen: &mut std::collections::HashMap<(usize, usize), (i32, i32)>,
    ) -> (i32, i32) {
        if start >= end {
            return (arr[start], 0);
        } else {
            if let Some((max_leaf, min_sum)) = seen.get(&(start, end)) {
                // hit memoized cache at key (start, end)
                return (max_leaf.clone(), min_sum.clone());
            } else {
                let mut min_sum = std::i32::MAX;
                let mut max_left: i32 = 0;
                let mut max_right: i32 = 0;

                // cut arr[start..=end] into arr[start..=mid] + arr[mid+1..=end] with different mid
                for mid in start..end {
                    let mut this_sum = 0;

                    let (left, left_sum) = Self::dfs_helper(arr, start, mid, seen);
                    let (right, right_sum) = Self::dfs_helper(arr, mid + 1, end, seen);
                    let total = left * right + left_sum + right_sum; // accumulate candidate min product sum here

                    if total < min_sum {
                        // candidate is best candidate
                        min_sum = total; // update min prod sum
                        max_left = left; // update max leaf
                        max_right = right;
                    }
                }
                seen.insert((start, end), (std::cmp::max(max_left, max_right), min_sum)); // cache (start, end) as key
                return (std::cmp::max(max_left, max_right), min_sum);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1130_example_1() {
        let arr = vec![6, 2, 4];
        let result = 32;

        assert_eq!(Solution::mct_from_leaf_values(arr), result);
    }

    #[test]
    fn test_1130_example_2() {
        let arr = vec![4, 11];
        let result = 44;

        assert_eq!(Solution::mct_from_leaf_values(arr), result);
    }
}
