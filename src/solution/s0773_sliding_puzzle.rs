/**
 * [0773] Sliding Puzzle
 *
 * On an 2 x 3 board, there are five tiles labeled from 1 to 5, and an empty square represented by 0. A move consists of choosing 0 and a 4-directionally adjacent number and swapping it.
 * The state of the board is solved if and only if the board is [[1,2,3],[4,5,0]].
 * Given the puzzle board board, return the least number of moves required so that the state of the board is solved. If it is impossible for the state of the board to be solved, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/slide1-grid.jpg" style="width: 244px; height: 165px;" />
 * Input: board = [[1,2,3],[4,0,5]]
 * Output: 1
 * Explanation: Swap the 0 and the 5 in one move.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/slide2-grid.jpg" style="width: 244px; height: 165px;" />
 * Input: board = [[1,2,3],[5,4,0]]
 * Output: -1
 * Explanation: No number of moves will make the board solved.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/slide3-grid.jpg" style="width: 244px; height: 165px;" />
 * Input: board = [[4,1,2],[5,0,3]]
 * Output: 5
 * Explanation: 5 is the smallest number of moves that solves the board.
 * An example path:
 * After move 0: [[4,1,2],[5,0,3]]
 * After move 1: [[4,1,2],[0,5,3]]
 * After move 2: [[0,1,2],[4,5,3]]
 * After move 3: [[1,0,2],[4,5,3]]
 * After move 4: [[1,2,0],[4,5,3]]
 * After move 5: [[1,2,3],[4,5,0]]
 *
 *  
 * Constraints:
 *
 * 	board.length == 2
 * 	board[i].length == 3
 * 	0 <= board[i][j] <= 5
 * 	Each value board[i][j] is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-puzzle/
// discuss: https://leetcode.com/problems/sliding-puzzle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    //Credit: https://leetcode.com/problems/sliding-puzzle/discuss/873598/Rust-translated-0ms-100
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let move_to = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];

        let target = vec![1, 2, 3, 4, 5, 0];
        let start: Vec<i32> = board.concat();
        let mut visisted = std::collections::HashSet::<Vec<i32>>::new();
        let mut queue = std::collections::VecDeque::new();

        visisted.insert(start.clone());
        queue.push_back(start);

        let mut result = 0;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let cur = queue.pop_front().unwrap();
                if cur == target {
                    return result;
                }
                let zero = cur.iter().position(|&x| x == 0).unwrap();
                for &x in &move_to[zero] {
                    let mut next = cur.to_vec();
                    next.swap(x, zero);
                    if visisted.contains(&next) {
                        continue;
                    } else {
                        visisted.insert(next.clone());
                        queue.push_back(next);
                    }
                }
            }
            result += 1;
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0773_example_1() {
        let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
        let result = 1;

        assert_eq!(Solution::sliding_puzzle(board), result);
    }

    #[test]
    fn test_0773_example_2() {
        let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
        let result = -1;

        assert_eq!(Solution::sliding_puzzle(board), result);
    }

    #[test]
    fn test_0773_example_3() {
        let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
        let result = 5;

        assert_eq!(Solution::sliding_puzzle(board), result);
    }
}
