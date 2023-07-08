/**
 * [1202] Smallest String With Swaps
 *
 * You are given a string s, and an array of pairs of indices in the string pairs where pairs[i] = [a, b] indicates 2 indices(0-indexed) of the string.
 * You can swap the characters at any pair of indices in the given pairs any number of times.
 * Return the lexicographically smallest string that s can be changed to after using the swaps.
 *  
 * Example 1:
 *
 * Input: s = "dcab", pairs = [[0,3],[1,2]]
 * Output: "bacd"
 * Explaination:
 * Swap s[0] and s[3], s = "bcad"
 * Swap s[1] and s[2], s = "bacd"
 *
 * Example 2:
 *
 * Input: s = "dcab", pairs = [[0,3],[1,2],[0,2]]
 * Output: "abcd"
 * Explaination:
 * Swap s[0] and s[3], s = "bcad"
 * Swap s[0] and s[2], s = "acbd"
 * Swap s[1] and s[2], s = "abcd"
 * Example 3:
 *
 * Input: s = "cba", pairs = [[0,1],[1,2]]
 * Output: "abc"
 * Explaination:
 * Swap s[0] and s[1], s = "bca"
 * Swap s[1] and s[2], s = "bac"
 * Swap s[0] and s[1], s = "abc"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	0 <= pairs.length <= 10^5
 * 	0 <= pairs[i][0], pairs[i][1] < s.length
 * 	s only contains lower case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-string-with-swaps/
// discuss: https://leetcode.com/problems/smallest-string-with-swaps/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            roots: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.find(self.roots[x]);
        }
        self.roots[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if self.ranks[x] > self.ranks[y] {
            self.roots[y] = x;
        } else if self.ranks[x] < self.ranks[y] {
            self.roots[x] = y;
        } else {
            self.roots[y] = x;
            self.ranks[y] += 1;
        }
    }
}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut uf = UnionFind::new(s.len());
        pairs
            .iter()
            .for_each(|p| uf.union(p[0] as usize, p[1] as usize));

        let mut s: Vec<char> = s.chars().collect();
        let mut map = std::collections::HashMap::new();
        for i in 0..s.len() {
            map.entry(uf.find(i)).or_insert(vec![]).push(i);
        }

        for (k, v) in map.iter() {
            let mut arr = Vec::new();
            v.iter().for_each(|&i| arr.push(s[i]));
            arr.sort();
            v.iter().zip(arr.into_iter()).for_each(|(&i, c)| s[i] = c);
        }

        s.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1202_example_1() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0, 3], vec![1, 2]];
        let result = "bacd".to_string();

        assert_eq!(Solution::smallest_string_with_swaps(s, pairs), result);
    }

    #[test]
    fn test_1202_example_2() {
        let s = "dcab".to_string();
        let pairs = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
        let result = "abcd".to_string();

        assert_eq!(Solution::smallest_string_with_swaps(s, pairs), result);
    }

    #[test]
    fn test_1202_example_3() {
        let s = "cba".to_string();
        let pairs = vec![vec![0, 1], vec![1, 2]];
        let result = "abc".to_string();

        assert_eq!(Solution::smallest_string_with_swaps(s, pairs), result);
    }
}
