/**
 * [1261] Find Elements in a Contaminated Binary Tree
 *
 * Given a binary tree with the following rules:
 * <ol>
 * 	root.val == 0
 * 	If treeNode.val == x and treeNode.left != null, then treeNode.left.val == 2 * x + 1
 * 	If treeNode.val == x and treeNode.right != null, then treeNode.right.val == 2 * x + 2
 * </ol>
 * Now the binary tree is contaminated, which means all treeNode.val have been changed to -1.
 * Implement the FindElements class:
 *
 * 	FindElements(TreeNode* root) Initializes the object with a contaminated binary tree and recovers it.
 * 	bool find(int target) Returns true if the target value exists in the recovered binary tree.
 *
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4-1.jpg" style="width: 320px; height: 119px;" />
 * Input
 * ["FindElements","find","find"]
 * [[[-1,null,-1]],[1],[2]]
 * Output
 * [null,false,true]
 * Explanation
 * FindElements findElements = new FindElements([-1,null,-1]);
 * findElements.find(1); // return False
 * findElements.find(2); // return True
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4.jpg" style="width: 400px; height: 198px;" />
 * Input
 * ["FindElements","find","find","find"]
 * [[[-1,-1,-1,-1,-1]],[1],[3],[5]]
 * Output
 * [null,true,true,false]
 * Explanation
 * FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
 * findElements.find(1); // return True
 * findElements.find(3); // return True
 * findElements.find(5); // return False
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/07/untitled-diagram-4-1-1.jpg" style="width: 306px; height: 274px;" />
 * Input
 * ["FindElements","find","find","find","find"]
 * [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
 * Output
 * [null,true,false,false,true]
 * Explanation
 * FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
 * findElements.find(2); // return True
 * findElements.find(3); // return False
 * findElements.find(4); // return False
 * findElements.find(5); // return True
 *
 *
 * Constraints:
 *
 * 	TreeNode.val == -1
 * 	The height of the binary tree is less than or equal to 20
 * 	The total number of nodes is between [1, 10^4]
 * 	Total calls of find() is between [1, 10^4]
 * 	0 <= target <= 10^6
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
// discuss: https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Default)]
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
    elements: std::collections::HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut elements = Self::default();
        elements.populate(&root, 0);
        elements
    }

    fn find(&self, target: i32) -> bool {
        self.elements.contains(&target)
    }

    fn populate(&mut self, root: &Option<Rc<RefCell<TreeNode>>>, index: i32) {
        if let Some(root_ref) = root {
            let mut root_node = root_ref.borrow_mut();
            root_node.val = index;
            self.elements.insert(index);
            self.populate(&root_node.left, index * 2 + 1);
            self.populate(&root_node.right, index * 2 + 2);
        }
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1261_example_1() {
        let mut find_elements = FindElements::new(tree![-1, null, -1, -1, null, -1]);
        assert_eq!(find_elements.find(2), true); // return True
        assert_eq!(find_elements.find(3), false); // return False
        assert_eq!(find_elements.find(4), false); // return False
        assert_eq!(find_elements.find(5), true); // return True
    }
}
