/**
 * [0756] Pyramid Transition Matrix
 *
 * You are stacking blocks to form a pyramid. Each block has a color, which is represented by a single letter. Each row of blocks contains one less block than the row beneath it and is centered on top.
 * To make the pyramid aesthetically pleasing, there are only specific triangular patterns that are allowed. A triangular pattern consists of a single block stacked on top of two blocks. The patterns are given as a list of three-letter strings allowed, where the first two characters of a pattern represent the left and right bottom blocks respectively, and the third character is the top block.
 *
 * 	For example, "ABC" represents a triangular pattern with a 'C' block stacked on top of an 'A' (left) and 'B' (right) block. Note that this is different from "BAC" where 'B' is on the left bottom and 'A' is on the right bottom.
 *
 * You start with a bottom row of blocks bottom, given as a single string, that you must use as the base of the pyramid.
 * Given bottom and allowed, return true if you can build the pyramid all the way to the top such that every triangular pattern in the pyramid is in allowed, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/26/pyramid1-grid.jpg" style="width: 600px; height: 232px;" />
 * Input: bottom = "BCD", allowed = ["BCC","CDE","CEA","FFF"]
 * Output: true
 * Explanation: The allowed triangular patterns are shown on the right.
 * Starting from the bottom (level 3), we can build "CE" on level 2 and then build "A" on level 1.
 * There are three triangular patterns in the pyramid, which are "BCC", "CDE", and "CEA". All are allowed.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/26/pyramid2-grid.jpg" style="width: 600px; height: 359px;" />
 * Input: bottom = "AAAA", allowed = ["AAB","AAC","BCD","BBE","DEF"]
 * Output: false
 * Explanation: The allowed triangular patterns are shown on the right.
 * Starting from the bottom (level 4), there are multiple ways to build level 3, but trying all the possibilites, you will get always stuck before building level 1.
 *
 *  
 * Constraints:
 *
 * 	2 <= bottom.length <= 6
 * 	0 <= allowed.length <= 216
 * 	allowed[i].length == 3
 * 	The letters in all input strings are from the set {'A', 'B', 'C', 'D', 'E', 'F'}.
 * 	All the values of allowed are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pyramid-transition-matrix/
// discuss: https://leetcode.com/problems/pyramid-transition-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let n = bottom.len();
        let mut v: Vec<Vec<usize>> = vec![vec![]; n];
        let mut map = vec![vec![std::collections::HashSet::new(); 7]; 7];
        for c in bottom.bytes() {
            let b = (c - b'A') as usize;
            v[n - 1].push(b);
        }
        for s in allowed {
            let s: Vec<u8> = s.bytes().collect();
            let a: usize = (s[0] - b'A') as usize;
            let b: usize = (s[1] - b'A') as usize;
            let c: usize = (s[2] - b'A') as usize;
            map[a][b].insert(c);
        }
        Self::dfs_helper(&mut v, &map, n - 1, n - 1)
    }

    fn dfs_helper(
        v: &mut Vec<Vec<usize>>,
        map: &[Vec<std::collections::HashSet<usize>>],
        row: usize,
        col: usize,
    ) -> bool {
        if row == 0 {
            return true;
        }
        let (r, c) = if col == row {
            (row - 1, 0)
        } else {
            (row, col + 1)
        };
        let left = v[r + 1][c];
        let right = v[r + 1][c + 1];
        for &x in &map[left][right] {
            v[r].push(x);
            if Self::dfs_helper(v, map, r, c) {
                return true;
            }
            v[r].pop();
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0756_example_1() {
        let bottom = "BCD".to_string();
        let allowed = vec_string!["BCC", "CDE", "CEA", "FFF"];
        let result = true;

        assert_eq!(Solution::pyramid_transition(bottom, allowed), result);
    }

    #[test]
    fn test_0756_example_2() {
        let bottom = "AAAA".to_string();
        let allowed = vec_string!["AAB", "AAC", "BCD", "BBE", "DEF"];
        let result = false;

        assert_eq!(Solution::pyramid_transition(bottom, allowed), result);
    }
}
