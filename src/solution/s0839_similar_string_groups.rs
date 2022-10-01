/**
 * [0839] Similar String Groups
 *
 * Two strings X and Y are similar if we can swap two letters (in different positions) of X, so that it equals Y. Also two strings X and Y are similar if they are equal.
 * For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".
 * Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.
 * We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?
 *  
 * Example 1:
 *
 * Input: strs = ["tars","rats","arts","star"]
 * Output: 2
 *
 * Example 2:
 *
 * Input: strs = ["omv","ovm"]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= strs.length <= 300
 * 	1 <= strs[i].length <= 300
 * 	strs[i] consists of lowercase letters only.
 * 	All words in strs have the same length and are anagrams of each other.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/similar-string-groups/
// discuss: https://leetcode.com/problems/similar-string-groups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    count: usize,
    parent: std::cell::RefCell<Vec<usize>>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            count: size,
            parent: std::cell::RefCell::new((0..size).collect()),
            size: vec![1; size],
        }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn find(&self, p: usize) -> usize {
        let mut root = p;
        while root != self.parent.borrow()[root] {
            root = self.parent.borrow()[root];
        }
        let mut p = p;
        while p != root {
            let next = self.parent.borrow()[p];
            self.parent.borrow_mut()[p] = root;
            p = next;
        }
        root
    }

    pub fn is_connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    pub fn connect(&mut self, p: usize, q: usize) {
        let (p_root, q_root) = (self.find(p), self.find(q));
        if p_root == q_root {
            return;
        }
        if self.size[p_root] < self.size[q_root] {
            self.parent.borrow_mut()[p_root] = q_root;
            self.size[q_root] += self.size[p_root];
        } else {
            self.parent.borrow_mut()[q_root] = p_root;
            self.size[p_root] += self.size[q_root];
        }
        self.count -= 1;
    }
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        fn is_similar(s1: &str, s2: &str) -> bool {
            s1.chars().zip(s2.chars()).filter(|(a, b)| !a.eq(b)).count() <= 2
        }
        let mut uf = UnionFind::new(strs.len());

        for i in 0..strs.len() {
            let s1 = strs.get(i).unwrap();
            for j in i + 1..strs.len() {
                if uf.is_connected(i, j) {
                    continue;
                }

                let s2 = strs.get(j).unwrap();
                if is_similar(s1, s2) {
                    uf.connect(i, j);
                }
            }
        }

        uf.count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0839_example_1() {
        let strs = vec_string!["tars", "rats", "arts", "star"];
        let result = 2;

        assert_eq!(Solution::num_similar_groups(strs), result);
    }

    #[test]
    fn test_0839_example_2() {
        let strs = vec_string!["omv", "ovm"];
        let result = 1;

        assert_eq!(Solution::num_similar_groups(strs), result);
    }
}
