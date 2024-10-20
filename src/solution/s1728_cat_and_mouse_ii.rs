/**
 * [1728] Cat and Mouse II
 *
 * A game is played by a cat and a mouse named Cat and Mouse.
 * The environment is represented by a grid of size rows x cols, where each element is a wall, floor, player (Cat, Mouse), or food.
 *
 * 	Players are represented by the characters 'C'(Cat),'M'(Mouse).
 * 	Floors are represented by the character '.' and can be walked on.
 * 	Walls are represented by the character '#' and cannot be walked on.
 * 	Food is represented by the character 'F' and can be walked on.
 * 	There is only one of each character 'C', 'M', and 'F' in grid.
 *
 * Mouse and Cat play according to the following rules:
 *
 * 	Mouse moves first, then they take turns to move.
 * 	During each turn, Cat and Mouse can jump in one of the four directions (left, right, up, down). They cannot jump over the wall nor outside of the grid.
 * 	catJump, mouseJump are the maximum lengths Cat and Mouse can jump at a time, respectively. Cat and Mouse can jump less than the maximum length.
 * 	Staying in the same position is allowed.
 * 	Mouse can jump over Cat.
 *
 * The game can end in 4 ways:
 *
 * 	If Cat occupies the same position as Mouse, Cat wins.
 * 	If Cat reaches the food first, Cat wins.
 * 	If Mouse reaches the food first, Mouse wins.
 * 	If Mouse cannot get to the food within 1000 turns, Cat wins.
 *
 * Given a rows x cols matrix grid and two integers catJump and mouseJump, return true if Mouse can win the game if both Cat and Mouse play optimally, otherwise return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/12/sample_111_1955.png" style="width: 580px; height: 239px;" />
 * Input: grid = ["####F","#C...","M...."], catJump = 1, mouseJump = 2
 * Output: true
 * Explanation: Cat cannot catch Mouse on its turn nor can it get the food before Mouse.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/12/sample_2_1955.png" style="width: 580px; height: 175px;" />
 * Input: grid = ["M.C...F"], catJump = 1, mouseJump = 4
 * Output: true
 *
 * Example 3:
 *
 * Input: grid = ["M.C...F"], catJump = 1, mouseJump = 3
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	rows == grid.length
 * 	cols = grid[i].length
 * 	1 <= rows, cols <= 8
 * 	grid[i][j] consist only of characters 'C', 'M', 'F', '.', and '#'.
 * 	There is only one of each character 'C', 'M', and 'F' in grid.
 * 	1 <= catJump, mouseJump <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cat-and-mouse-ii/
// discuss: https://leetcode.com/problems/cat-and-mouse-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        // TODO
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1728_example_1() {
        let grid = vec_string!["####F", "#C...", "M...."];
        let cat_jump = 1;
        let mouse_jump = 2;

        let result = true;

        assert_eq!(Solution::can_mouse_win(grid, cat_jump, mouse_jump), result);
    }

    #[test]
    #[ignore]
    fn test_1728_example_2() {
        let grid = vec_string!["M.C...F"];
        let cat_jump = 1;
        let mouse_jump = 4;

        let result = true;

        assert_eq!(Solution::can_mouse_win(grid, cat_jump, mouse_jump), result);
    }

    #[test]
    #[ignore]
    fn test_1728_example_3() {
        let grid = vec_string!["M.C...F"];
        let cat_jump = 1;
        let mouse_jump = 3;

        let result = false;

        assert_eq!(Solution::can_mouse_win(grid, cat_jump, mouse_jump), result);
    }
}
