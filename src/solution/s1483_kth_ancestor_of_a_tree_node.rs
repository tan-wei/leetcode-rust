/**
 * [1483] Kth Ancestor of a Tree Node
 *
 * You are given a tree with n nodes numbered from 0 to n - 1 in the form of a parent array parent where parent[i] is the parent of i^th node. The root of the tree is node 0. Find the k^th ancestor of a given node.
 * The k^th ancestor of a tree node is the k^th node in the path from that node to the root node.
 * Implement the TreeAncestor class:
 *
 * 	TreeAncestor(int n, int[] parent) Initializes the object with the number of nodes in the tree and the parent array.
 * 	int getKthAncestor(int node, int k) return the k^th ancestor of the given node node. If there is no such ancestor, return -1.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/28/1528_ex1.png" style="width: 396px; height: 262px;" />
 * Input
 * ["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
 * [[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
 * Output
 * [null, 1, 0, -1]
 * Explanation
 * TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
 * treeAncestor.getKthAncestor(3, 1); // returns 1 which is the parent of 3
 * treeAncestor.getKthAncestor(5, 2); // returns 0 which is the grandparent of 5
 * treeAncestor.getKthAncestor(6, 3); // returns -1 because there is no such ancestor
 *  
 * Constraints:
 *
 * 	1 <= k <= n <= 5 * 10^4
 * 	parent.length == n
 * 	parent[0] == -1
 * 	0 <= parent[i] < n for all 0 < i < n
 * 	0 <= node < n
 * 	There will be at most 5 * 10^4 queries.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-ancestor-of-a-tree-node/
// discuss: https://leetcode.com/problems/kth-ancestor-of-a-tree-node/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct TreeAncestor {
    parent: Vec<[i32; 31]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut par = vec![[-1; 31]; n as usize];
        for i in 0..n as usize {
            par[i][0] = parent[i];
        }
        for j in 1..31 {
            for i in 1..n as usize {
                let p = par[i][j - 1];
                if p != -1 {
                    par[i][j] = par[p as usize][j - 1];
                }
            }
        }
        Self { parent: par }
    }

    fn get_kth_ancestor(&mut self, node: i32, k: i32) -> i32 {
        let mut i = 0;
        let mut node = node;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                node = self.parent[node as usize][i];
                if node == -1 {
                    break;
                }
            }
            k >>= 1;
            i += 1;
        }
        node
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1483_example_1() {
        let mut tree_ancestor = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(tree_ancestor.get_kth_ancestor(3, 1), 1); // returns 1 which is the parent of 3
        assert_eq!(tree_ancestor.get_kth_ancestor(5, 2), 0); // returns 0 which is the grandparent of 5
        assert_eq!(tree_ancestor.get_kth_ancestor(6, 3), -1); // returns -1 because there is no such ancestor
    }
}
