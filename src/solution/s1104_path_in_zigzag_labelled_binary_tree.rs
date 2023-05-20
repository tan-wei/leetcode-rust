/**
 * [1104] Path In Zigzag Labelled Binary Tree
 *
 * In an infinite binary tree where every node has two children, the nodes are labelled in row order.
 * In the odd numbered rows (ie., the first, third, fifth,...), the labelling is left to right, while in the even numbered rows (second, fourth, sixth,...), the labelling is right to left.
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/24/tree.png" style="width: 300px; height: 138px;" />
 * Given the label of a node in this tree, return the labels in the path from the root of the tree to the node with that label.
 *  
 * Example 1:
 *
 * Input: label = 14
 * Output: [1,3,4,14]
 *
 * Example 2:
 *
 * Input: label = 26
 * Output: [1,2,6,10,26]
 *
 *  
 * Constraints:
 *
 * 	1 <= label <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/
// discuss: https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut result = vec![label];
        let mut value: i32 = label;
        while value != 1 {
            value = (value >> 1) ^ (1 << format!("{:b}", (value >> 1)).chars().count() - 1) - 1;
            result.push(value);
        }
        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1104_example_1() {
        let label = 14;
        let result = vec![1, 3, 4, 14];

        assert_eq!(Solution::path_in_zig_zag_tree(label), result);
    }

    #[test]
    fn test_1104_example_2() {
        let label = 26;
        let result = vec![1, 2, 6, 10, 26];

        assert_eq!(Solution::path_in_zig_zag_tree(label), result);
    }
}
