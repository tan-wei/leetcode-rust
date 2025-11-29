/**
 * [2257] Count Unguarded Cells in the Grid
 *
 * You are given two integers m and n representing a 0-indexed m x n grid. You are also given two 2D integer arrays guards and walls where guards[i] = [rowi, coli] and walls[j] = [rowj, colj] represent the positions of the i^th guard and j^th wall respectively.
 * A guard can see every cell in the four cardinal directions (north, east, south, or west) starting from their position unless obstructed by a wall or another guard. A cell is guarded if there is at least one guard that can see it.
 * Return the number of unoccupied cells that are not guarded.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/10/example1drawio2.png" style="width: 300px; height: 204px;" />
 * Input: m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
 * Output: 7
 * Explanation: The guarded and unguarded cells are shown in red and green respectively in the above diagram.
 * There are a total of 7 unguarded cells, so we return 7.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/10/example2drawio.png" style="width: 200px; height: 201px;" />
 * Input: m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
 * Output: 4
 * Explanation: The unguarded cells are shown in green in the above diagram.
 * There are a total of 4 unguarded cells, so we return 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 10^5
 * 	2 <= m * n <= 10^5
 * 	1 <= guards.length, walls.length <= 5 * 10^4
 * 	2 <= guards.length + walls.length <= m * n
 * 	guards[i].length == walls[j].length == 2
 * 	0 <= rowi, rowj < m
 * 	0 <= coli, colj < n
 * 	All the positions in guards and walls are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-unguarded-cells-in-the-grid/
// discuss: https://leetcode.com/problems/count-unguarded-cells-in-the-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2257_example_1() {
        let m = 4;
        let n = 6;
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];

        let result = 7;

        assert_eq!(Solution::count_unguarded(m, n, guards, walls), result);
    }

    #[test]
    #[ignore]
    fn test_2257_example_2() {
        let m = 3;
        let n = 3;
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];

        let result = 4;

        assert_eq!(Solution::count_unguarded(m, n, guards, walls), result);
    }
}
