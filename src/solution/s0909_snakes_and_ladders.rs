/**
 * [0909] Snakes and Ladders
 *
 * You are given an n x n integer matrix board where the cells are labeled from 1 to n^2 in a <a href="https://en.wikipedia.org/wiki/Boustrophedon" target="_blank">Boustrophedon style</a> starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.
 * You start on square 1 of the board. In each move, starting from square curr, do the following:
 *
 * 	Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n^2)].
 *
 * 		This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
 *
 *
 * 	If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
 * 	The game ends when you reach the square n^2.
 *
 * A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that snake or ladder is board[r][c]. Squares 1 and n^2 do not have a snake or ladder.
 * Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.
 *
 * 	For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
 *
 * Return the least number of moves required to reach the square n^2. If it is not possible to reach the square, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/09/23/snakes.png" style="width: 500px; height: 394px;" />
 * Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
 * Output: 4
 * Explanation:
 * In the beginning, you start at square 1 (at row 5, column 0).
 * You decide to move to square 2 and must take the ladder to square 15.
 * You then decide to move to square 17 and must take the snake to square 13.
 * You then decide to move to square 14 and must take the ladder to square 35.
 * You then decide to move to square 36, ending the game.
 * This is the lowest possible number of moves to reach the last square, so return 4.
 *
 * Example 2:
 *
 * Input: board = [[-1,-1],[-1,3]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == board.length == board[i].length
 * 	2 <= n <= 20
 * 	grid[i][j] is either -1 or in the range [1, n^2].
 * 	The squares labeled 1 and n^2 do not have any ladders or snakes.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/snakes-and-ladders/
// discuss: https://leetcode.com/problems/snakes-and-ladders/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut queue = std::collections::VecDeque::<i32>::new();
        queue.push_back(1);
        let mut visited = vec![false; n * n + 1];
        let mut result = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let num = queue.pop_front().unwrap();
                if visited[num as usize] {
                    continue;
                }
                visited[num as usize] = true;
                if num as usize == n * n {
                    return result;
                }
                (1..=6).for_each(|i| {
                    let mut next = num + i;
                    if next <= (n * n) as i32 {
                        let value = Self::get_board_value(&board, next);
                        if value > 0 {
                            next = value;
                        }
                        if !visited[next as usize] {
                            queue.push_back(next);
                        }
                    }
                });
            }
            result += 1;
        }
        -1
    }

    fn get_board_value(board: &[Vec<i32>], num: i32) -> i32 {
        let n = board.len() as i32;
        let r = (num - 1) / n;
        let x = n - 1 - r;
        let y = if r % 2 == 0 {
            num - 1 - r * n
        } else {
            n + r * n - num
        };
        board[x as usize][y as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0909_example_1() {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        let result = 4;

        assert_eq!(Solution::snakes_and_ladders(board), result);
    }

    #[test]
    fn test_0909_example_2() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        let result = 1;

        assert_eq!(Solution::snakes_and_ladders(board), result);
    }
}
