/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
 * Clarification: The input/output format is the same as <a href="/faq/#binary-tree">how LeetCode serializes a binary tree</a>. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg" style="width: 442px; height: 324px;" />
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 * Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 *
 * Example 4:
 *
 * Input: root = [1,2]
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
// discuss: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = vec![];
        Self::preorder_dfs(root, &mut res);
        res.join(",")
    }

    fn preorder_dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>) {
        if let Some(ref a) = node {
            res.push(a.borrow().val.to_string());
            Self::preorder_dfs(a.borrow().left.clone(), res);
            Self::preorder_dfs(a.borrow().right.clone(), res);
        } else {
            res.push("null".to_string())
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let lst: Vec<Option<i32>> = data
            .split(",")
            .map(|s| {
                if s == "null" {
                    None
                } else {
                    Some(s.parse::<i32>().unwrap())
                }
            })
            .collect();
        let mut cur = 0;
        Self::build_tree(&lst, &mut cur)
    }

    fn build_tree(src: &[Option<i32>], cur: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *cur >= src.len() {
            return None;
        }
        if let Some(val) = src[*cur] {
            *cur += 1;
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::build_tree(src, cur),
                right: Self::build_tree(src, cur),
            })))
        } else {
            *cur += 1;
            None
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0297_example_1() {
        let root = tree![1, 2, 3, null, null, 4, 5];
        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        let result = codec.deserialize(data);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0297_example_2() {
        let root = tree![];
        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        let result = codec.deserialize(data);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0297_example_3() {
        let root = tree![1];
        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        let result = codec.deserialize(data);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0297_example_4() {
        let root = tree![1, 2];
        let codec = Codec::new();
        let data = codec.serialize(root.clone());
        let result = codec.deserialize(data);

        assert_eq!(root, result);
    }
}
