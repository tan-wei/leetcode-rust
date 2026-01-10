/**
 * [0449] Serialize and Deserialize BST
 *
 * Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary search tree. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.
 * The encoded string should be as compact as possible.
 *  
 * Example 1:
 * Input: root = [2,1,3]
 * Output: [2,1,3]
 * Example 2:
 * Input: root = []
 * Output: []
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	0 <= Node.val <= 10^4
 * 	The input tree is guaranteed to be a binary search tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/serialize-and-deserialize-bst/
// discuss: https://leetcode.com/problems/serialize-and-deserialize-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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

// Credit: https://leetcode.com/problems/serialize-and-deserialize-bst/discuss/897833/Rust-4ms-100(without-check-validation)
///
/// get "Pointer" of a Tree Node
fn to_rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => Some(Rc::clone(node)),
        None => None,
    }
}

///
/// get value of a Tree Node
fn val_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        Some(node) => {
            let node = node.borrow();
            Some(node.val)
        }
        None => None,
    }
}

///
/// get left of a Tree Node
fn left_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let node = node.borrow();
            match &node.left {
                Some(l) => Some(Rc::clone(l)),
                None => None,
            }
        }
        None => None,
    }
}

///
/// get right of a Tree Node
fn right_of(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let node = node.borrow();
            match &node.right {
                Some(r) => Some(Rc::clone(r)),
                None => None,
            }
        }
        None => None,
    }
}

/// root is not null
/// left is none or left has no child
fn append_to_left(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.left = to_rc(&ans);
        }
        None => {}
    }
    ans
}

/// root is not null
/// right is none or right has no child
fn append_to_right(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    match root {
        Some(node) => {
            let mut node = node.borrow_mut();
            ans = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.right = to_rc(&ans);
        }
        None => {}
    }
    ans
}

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
        let mut ans = String::new();
        ans.push('[');
        let mut q = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        if root.is_some() {
            q.push_back(to_rc(&root));
            ans.push_str(&val_of(&root).unwrap().to_string());
            ans.push(',');
        }
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            let left = left_of(&node);
            let right = right_of(&node);
            if left.is_none() {
                ans.push_str("null,");
            } else {
                ans.push_str(&val_of(&left).unwrap().to_string());
                ans.push(',');
                q.push_back(left);
            }
            if right.is_none() {
                ans.push_str("null,");
            } else {
                ans.push_str(&val_of(&right).unwrap().to_string());
                ans.push(',');
                q.push_back(right);
            }
        }
        while ans.len() > 5 && &ans[ans.len() - 5..] == "null," {
            ans.truncate(ans.len() - 5);
        }
        match ans.pop() {
            Some(ch) if ch == ',' => {}
            Some(ch) => ans.push(ch),
            None => {}
        }
        ans.push(']');
        ans
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() < 3 {
            return None;
        }
        let strs = data[1..data.len() - 1]
            .split(',')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let n = strs.len();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            strs[0].parse::<i32>().unwrap(),
        ))));

        let mut q = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        q.push_back(to_rc(&root));
        let mut i = 1;
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let val = val_of(&node).unwrap();

                if i < n && strs[i] != "null" {
                    let new_val = strs[i].parse::<i32>().unwrap();
                    if new_val < val {
                        let left = append_to_left(&node, new_val);
                        q.push_back(left);
                    } else if new_val > val {
                        let right = append_to_right(&node, new_val);
                        q.push_back(right);
                    }
                };
                i += 1;
                if i < n && strs[i] != "null" {
                    let new_val = strs[i].parse::<i32>().unwrap();
                    if new_val < val {
                        let left = append_to_left(&node, new_val);
                        q.push_back(left);
                    } else if new_val > val {
                        let right = append_to_right(&node, new_val);
                        q.push_back(right);
                    }
                }
                i += 1;
            }
        }

        root
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
    fn test_0449_example_1() {
        let codec = Codec::new();
        let s = "[2,1,3]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_0449_example_2() {
        let codec = Codec::new();
        let s = "[]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_0449_example_3() {
        let codec = Codec::new();
        let s = "[1]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_0449_example_4() {
        let codec = Codec::new();
        let s = "[1,null,2]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_0449_example_5() {
        let codec = Codec::new();
        let s = "[4,3,null,2,null,1]".to_owned();
        let root = codec.deserialize(s.clone());
        let s2 = codec.serialize(root);
        assert_eq!(s, s2);
    }
}
