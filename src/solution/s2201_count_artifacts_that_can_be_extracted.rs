/**
 * [2201] Count Artifacts That Can Be Extracted
 *
 * There is an n x n 0-indexed grid with some artifacts buried in it. You are given the integer n and a 0-indexed 2D integer array artifacts describing the positions of the rectangular artifacts where artifacts[i] = [r1i, c1i, r2i, c2i] denotes that the i^th artifact is buried in the subgrid where:
 *
 * 	(r1i, c1i) is the coordinate of the top-left cell of the i^th artifact and
 * 	(r2i, c2i) is the coordinate of the bottom-right cell of the i^th artifact.
 *
 * You will excavate some cells of the grid and remove all the mud from them. If the cell has a part of an artifact buried underneath, it will be uncovered. If all the parts of an artifact are uncovered, you can extract it.
 * Given a 0-indexed 2D integer array dig where dig[i] = [ri, ci] indicates that you will excavate the cell (ri, ci), return the number of artifacts that you can extract.
 * The test cases are generated such that:
 *
 * 	No two artifacts overlap.
 * 	Each artifact only covers at most 4 cells.
 * 	The entries of dig are unique.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/16/untitled-diagram.jpg" style="width: 216px; height: 216px;" />
 * Input: n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1]]
 * Output: 1
 * Explanation:
 * The different colors represent different artifacts. Excavated cells are labeled with a 'D' in the grid.
 * There is 1 artifact that can be extracted, namely the red artifact.
 * The blue artifact has one part in cell (1,1) which remains uncovered, so we cannot extract it.
 * Thus, we return 1.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/16/untitled-diagram-1.jpg" style="width: 216px; height: 216px;" />
 * Input: n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1],[1,1]]
 * Output: 2
 * Explanation: Both the red and blue artifacts have all parts uncovered (labeled with a 'D') and can be extracted, so we return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 * 	1 <= artifacts.length, dig.length <= min(n^2, 10^5)
 * 	artifacts[i].length == 4
 * 	dig[i].length == 2
 * 	0 <= r1i, c1i, r2i, c2i, ri, ci <= n - 1
 * 	r1i <= r2i
 * 	c1i <= c2i
 * 	No two artifacts will overlap.
 * 	The number of cells covered by an artifact is at most 4.
 * 	The entries of dig are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-artifacts-that-can-be-extracted/
// discuss: https://leetcode.com/problems/count-artifacts-that-can-be-extracted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2201_example_1() {
        let n = 2;
        let artifacts = vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]];
        let dig = vec![vec![0, 0], vec![0, 1]];

        let result = 1;

        assert_eq!(Solution::dig_artifacts(n, artifacts, dig), result);
    }

    #[test]
    #[ignore]
    fn test_2201_example_2() {
        let n = 2;
        let artifacts = vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]];
        let dig = vec![vec![0, 0], vec![0, 1], vec![1, 1]];

        let result = 2;

        assert_eq!(Solution::dig_artifacts(n, artifacts, dig), result);
    }
}
