/**
 * [37] Sudoku Solver
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 * A sudoku solution must satisfy all of the following rules:
 * <ol>
 * 	Each of the digits 1-9 must occur exactly once in each row.
 * 	Each of the digits 1-9 must occur exactly once in each column.
 * 	Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * </ol>
 * The '.' character indicates empty cells.
 *  
 * Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * Explanation: The input board is shown above and the only valid solution is shown below:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px" />
 *
 *  
 * Constraints:
 *
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] is a digit or '.'.
 * 	It is guaranteed that the input board has only one solution.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sudoku-solver/
// discuss: https://leetcode.com/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sudoku-solver/discuss/300781/Not-so-fast-but-easy-to-understand-Rust-solution
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board: Vec<_> = board.iter_mut().map(Vec::as_mut_slice).collect();

        Self::solve_backtrack(board.as_mut_slice(), 0);
    }

    /// The actual solving algorithm. Checks for each cell.
    fn solve_backtrack(grid: &mut [&mut [char]], cell: usize) -> bool {
        match (cell / 9, cell % 9) {
            // Terminal condition (all the lines were checked):
            (9, _) => true,
            // The cell has already a number, check the next cell:
            (y, x) if grid[y][x] != '.' => Self::solve_backtrack(grid, cell + 1),
            // Check for every number, and go for the next cell if the grid is valid:
            (y, x) => "123456789.".chars().any(|c| {
                grid[y][x] = c;
                c != '.' && Self::is_grid_valid(grid, y, x) && Self::solve_backtrack(grid, cell + 1)
            }),
        }
    }

    /// Checks if the grid is valid when we write a number at the position (y, x).
    /// It checks in the right line, column and square.
    fn is_grid_valid(grid: &mut [&mut [char]], y: usize, x: usize) -> bool {
        fn uniq(chs: impl Iterator<Item = char>, ch: char) -> bool {
            chs.filter(|&c| c == ch).count() == 1
        }

        let c = grid[y][x];
        let line = grid[y].iter().cloned();
        let column = grid.iter().map(|l| l[x]);
        let (x, y) = (x / 3 * 3, y / 3 * 3);
        let square = grid[y..y + 3].iter().flat_map(|l| &l[x..x + 3]).cloned();

        uniq(line, c) && uniq(column, c) && uniq(square, c)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0037_example_1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let result = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(board, result);
    }
}
