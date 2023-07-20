/**
 * [1222] Queens That Can Attack the King
 *
 * On a 0-indexed 8 x 8 chessboard, there can be multiple black queens ad one white king.
 * You are given a 2D integer array queens where queens[i] = [xQueeni, yQueeni] represents the position of the i^th black queen on the chessboard. You are also given an integer array king of length 2 where king = [xKing, yKing] represents the position of the white king.
 * Return the coordinates of the black queens that can directly attack the king. You may return the answer in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/21/chess1.jpg" style="width: 400px; height: 400px;" />
 * Input: queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]], king = [0,0]
 * Output: [[0,1],[1,0],[3,3]]
 * Explanation: The diagram above shows the three queens that can directly attack the king and the three queens that cannot attack the king (i.e., marked with red dashes).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/21/chess2.jpg" style="width: 400px; height: 400px;" />
 * Input: queens = [[0,0],[1,1],[2,2],[3,4],[3,5],[4,4],[4,5]], king = [3,3]
 * Output: [[2,2],[3,4],[4,4]]
 * Explanation: The diagram above shows the three queens that can directly attack the king and the three queens that cannot attack the king (i.e., marked with red dashes).
 *
 *  
 * Constraints:
 *
 * 	1 <= queens.length < 64
 * 	queens[i].length == king.length == 2
 * 	0 <= xQueeni, yQueeni, xKing, yKing < 8
 * 	All the given positions are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/queens-that-can-attack-the-king/
// discuss: https://leetcode.com/problems/queens-that-can-attack-the-king/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        const DIR: [(i32, i32); 8] = [
            (-1, 0),
            (-1, 1),
            (-1, -1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, 0),
            (1, -1),
        ];

        let mut board = vec![vec![0; 8]; 8];

        for q in queens {
            let x = q[0];
            let y = q[1];
            board[x as usize][y as usize] = 1;
        }

        let mut result = vec![];

        for d in DIR {
            Self::dfs_helper(&board, (king[0], king[1]), d, &mut result);
        }

        result
    }

    fn dfs_helper(
        board: &Vec<Vec<i32>>,
        position: (i32, i32),
        direction: (i32, i32),
        result: &mut Vec<Vec<i32>>,
    ) {
        let (x, y) = position;

        if x < 0 || y < 0 || x >= board.len() as i32 || y >= board[0].len() as i32 {
            return;
        }

        if board[x as usize][y as usize] == 1 {
            result.push(vec![x, y]);
            return;
        }

        let next_position = (x + direction.0, y + direction.1);
        Self::dfs_helper(board, next_position, direction, result);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1222_example_1() {
        let queens = vec![
            vec![0, 1],
            vec![1, 0],
            vec![4, 0],
            vec![0, 4],
            vec![3, 3],
            vec![2, 4],
        ];
        let king = vec![0, 0];
        let result = vec![vec![0, 1], vec![1, 0], vec![3, 3]];

        assert_eq_sorted!(Solution::queens_attackthe_king(queens, king), result);
    }

    #[test]
    fn test_1222_example_2() {
        let queens = vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![3, 5],
            vec![4, 4],
            vec![4, 5],
        ];
        let king = vec![3, 3];
        let result = vec![vec![2, 2], vec![3, 4], vec![4, 4]];

        assert_eq_sorted!(Solution::queens_attackthe_king(queens, king), result);
    }
}
