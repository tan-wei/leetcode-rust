/**
 * [0450] Delete Node in a BST
 *
 * Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
 * Basically, the deletion can be divided into two stages:
 * <ol>
 * 	Search for a node to remove.
 * 	If the node is found, delete the node.
 * </ol>
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/04/del_node_1.jpg" style="width: 800px; height: 214px;" />
 * Input: root = [5,3,6,2,4,null,7], key = 3
 * Output: [5,4,6,2,null,null,7]
 * Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
 * One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
 * Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/04/del_node_supp.jpg" style="width: 350px; height: 255px;" />
 *
 * Example 2:
 *
 * Input: root = [5,3,6,2,4,null,7], key = 0
 * Output: [5,3,6,2,4,null,7]
 * Explanation: The tree does not contain a node with value = 0.
 *
 * Example 3:
 *
 * Input: root = [], key = 0
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 	Each node has a unique value.
 * 	root is a valid binary search tree.
 * 	-10^5 <= key <= 10^5
 *
 *  
 * Follow up: Could you solve it with time complexity O(height of tree)?
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/delete-node-in-a-bst/
// discuss: https://leetcode.com/problems/delete-node-in-a-bst/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::delete_node_helper(&root, key)
    }

    fn delete_node_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let val = n.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Greater => {
                    let l = Solution::delete_node_helper(&n.borrow().left, key);
                    n.borrow_mut().left = l;
                }
                std::cmp::Ordering::Less => {
                    let r = Solution::delete_node_helper(&n.borrow().right, key);
                    n.borrow_mut().right = r;
                }
                std::cmp::Ordering::Equal => {
                    if n.borrow().left.is_none() {
                        return n.borrow().right.clone();
                    }
                    if n.borrow().right.is_none() {
                        return n.borrow().left.clone();
                    }
                    let next = Solution::search_next(&n.borrow().right);
                    if let Some(val) = next {
                        let r = Solution::delete_node_helper(&n.borrow().right, val);
                        n.borrow_mut().val = val;
                        n.borrow_mut().right = r;
                    }
                }
            }
        }
        node.clone()
    }

    fn search_next(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Solution::search_next(&n.borrow().left)
            } else {
                Some(n.borrow().val)
            }
        } else {
            None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0450_example_1() {
        let root = tree![5, 3, 6, 2, 4, null, 7];
        let key = 3;
        let result = tree![5, 4, 6, 2, null, null, 7];

        assert_eq!(Solution::delete_node(root, key), result);
    }

    #[test]
    fn test_0450_example_2() {
        let root = tree![5, 3, 6, 2, 4, null, 7];
        let key = 0;
        let result = tree![5, 3, 6, 2, 4, null, 7];

        assert_eq!(Solution::delete_node(root, key), result);
    }

    #[test]
    fn test_0450_example_3() {
        let root = tree![];
        let key = 0;
        let result = tree![];

        assert_eq!(Solution::delete_node(root, key), result);
    }
}
