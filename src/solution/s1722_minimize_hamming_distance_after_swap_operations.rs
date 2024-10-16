/**
 * [1722] Minimize Hamming Distance After Swap Operations
 *
 * You are given two integer arrays, source and target, both of length n. You are also given an array allowedSwaps where each allowedSwaps[i] = [ai, bi] indicates that you are allowed to swap the elements at index ai and index bi (0-indexed) of array source. Note that you can swap elements at a specific pair of indices multiple times and in any order.
 * The Hamming distance of two arrays of the same length, source and target, is the number of positions where the elements are different. Formally, it is the number of indices i for 0 <= i <= n-1 where source[i] != target[i] (0-indexed).
 * Return the minimum Hamming distance of source and target after performing any amount of swap operations on array source.
 *  
 * Example 1:
 *
 * Input: source = [1,2,3,4], target = [2,1,4,5], allowedSwaps = [[0,1],[2,3]]
 * Output: 1
 * Explanation: source can be transformed the following way:
 * - Swap indices 0 and 1: source = [<u>2</u>,<u>1</u>,3,4]
 * - Swap indices 2 and 3: source = [2,1,<u>4</u>,<u>3</u>]
 * The Hamming distance of source and target is 1 as they differ in 1 position: index 3.
 *
 * Example 2:
 *
 * Input: source = [1,2,3,4], target = [1,3,2,4], allowedSwaps = []
 * Output: 2
 * Explanation: There are no allowed swaps.
 * The Hamming distance of source and target is 2 as they differ in 2 positions: index 1 and index 2.
 *
 * Example 3:
 *
 * Input: source = [5,1,2,4,3], target = [1,5,4,2,3], allowedSwaps = [[0,4],[4,2],[1,3],[1,4]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == source.length == target.length
 * 	1 <= n <= 10^5
 * 	1 <= source[i], target[i] <= 10^5
 * 	0 <= allowedSwaps.length <= 10^5
 * 	allowedSwaps[i].length == 2
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/
// discuss: https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/solutions/3199891/just-a-runnable-solution/

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parents = vec![0; n];
        for (i, item) in parents.iter_mut().enumerate().take(n) {
            *item = i;
        }
        UnionFind { parents }
    }
    fn find(&mut self, i: usize) -> usize {
        let mut i = i;
        while i != self.parents[i] {
            self.parents[i] = self.parents[self.parents[i]];
            i = self.parents[i];
        }
        i
    }
    fn union(&mut self, i: usize, j: usize) {
        let v = self.find(i);
        self.parents[v] = self.find(j);
    }
}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let mut uf = UnionFind::new(source.len());
        for s in allowed_swaps {
            uf.union(s[0] as usize, s[1] as usize);
        }

        let mut circles = std::collections::HashMap::new();
        for i in 0..source.len() {
            if source[i] == target[i] {
                continue;
            }
            let p = uf.find(i);
            circles
                .entry(p)
                .or_insert(std::collections::HashMap::new())
                .entry(source[i])
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let mut result = 0;
        for i in 0..source.len() {
            if source[i] == target[i] {
                continue;
            }
            let pi = uf.find(i);
            if circles.get(&pi).unwrap().get(&target[i]).unwrap_or(&0) == &0 {
                result += 1;
            } else {
                *circles.get_mut(&pi).unwrap().get_mut(&target[i]).unwrap() -= 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1722_example_1() {
        let source = vec![1, 2, 3, 4];
        let target = vec![2, 1, 4, 5];
        let allowed_swaps = vec![vec![0, 1], vec![2, 3]];

        let result = 1;

        assert_eq!(
            Solution::minimum_hamming_distance(source, target, allowed_swaps),
            result
        );
    }

    fn test_1722_example_2() {
        let source = vec![1, 2, 3, 4];
        let target = vec![1, 3, 2, 4];
        let allowed_swaps = vec![];

        let result = 2;

        assert_eq!(
            Solution::minimum_hamming_distance(source, target, allowed_swaps),
            result
        );
    }

    fn test_1722_example_3() {
        let source = vec![5, 1, 2, 4, 3];
        let target = vec![1, 5, 4, 2, 3];
        let allowed_swaps = vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]];

        let result = 0;

        assert_eq!(
            Solution::minimum_hamming_distance(source, target, allowed_swaps),
            result
        );
    }
}
