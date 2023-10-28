/**
 * [1361] Validate Binary Tree Nodes
 *
 * You have n binary tree nodes numbered from 0 to n - 1 where node i has two children leftChild[i] and rightChild[i], return true if and only if all the given nodes form exactly one valid binary tree.
 * If node i has no left child then leftChild[i] will equal -1, similarly for the right child.
 * Note that the nodes have no values and that we only use the node numbers in this problem.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex1.png" style="width: 195px; height: 287px;" />
 * Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,-1,-1,-1]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex2.png" style="width: 183px; height: 272px;" />
 * Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,3,-1,-1]
 * Output: false
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex3.png" style="width: 82px; height: 174px;" />
 * Input: n = 2, leftChild = [1,0], rightChild = [-1,-1]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	n == leftChild.length == rightChild.length
 * 	1 <= n <= 10^4
 * 	-1 <= leftChild[i], rightChild[i] <= n - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/validate-binary-tree-nodes/
// discuss: https://leetcode.com/problems/validate-binary-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut sum: i32 = 0;

        for i in 0..left_child.len() {
            if (left_child[i] != -1) {
                sum += left_child[i]
            }
            if (right_child[i] != -1) {
                sum += right_child[i]
            }
        }

        let mut q = std::collections::VecDeque::new();
        let root: i32 = n * (n - 1) / 2 - sum;
        if (root < 0 || root >= n) {
            return false;
        } else {
            q.push_back(root)
        }

        let mut v = std::collections::HashSet::new();
        while let Some(i) = q.pop_front() {
            let i = i as usize;

            if (v.contains(&i)) {
                return false;
            }

            v.insert(i);

            if (left_child[i] != -1) {
                q.push_back(left_child[i])
            }
            if (right_child[i] != -1) {
                q.push_back(right_child[i])
            }
        }

        v.len() == n as usize
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1361_example_1() {
        let n = 4;
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, -1, -1, -1];

        let result = true;

        assert_eq!(
            Solution::validate_binary_tree_nodes(n, left_child, right_child),
            result
        );
    }

    #[test]
    fn test_1361_example_2() {
        let n = 4;
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, 3, -1, -1];

        let result = false;

        assert_eq!(
            Solution::validate_binary_tree_nodes(n, left_child, right_child),
            result
        );
    }

    #[test]
    fn test_1361_example_3() {
        let n = 2;
        let left_child = vec![1, 0];
        let right_child = vec![-1, -1];

        let result = false;

        assert_eq!(
            Solution::validate_binary_tree_nodes(n, left_child, right_child),
            result
        );
    }
}
