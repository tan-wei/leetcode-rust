/**
 * [1812] Determine Color of a Chessboard Square
 *
 * You are given coordinates, a string that represents the coordinates of a square of the chessboard. Below is a chessboard for your reference.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/screenshot-2021-02-20-at-22159-pm.png" style="width: 400px; height: 396px;" />
 * Return true if the square is white, and false if the square is black.
 * The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first, and the number second.
 *  
 * Example 1:
 *
 * Input: coordinates = "a1"
 * Output: false
 * Explanation: From the chessboard above, the square with coordinates "a1" is black, so return false.
 *
 * Example 2:
 *
 * Input: coordinates = "h3"
 * Output: true
 * Explanation: From the chessboard above, the square with coordinates "h3" is white, so return true.
 *
 * Example 3:
 *
 * Input: coordinates = "c7"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	coordinates.length == 2
 * 	'a' <= coordinates[0] <= 'h'
 * 	'1' <= coordinates[1] <= '8'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-color-of-a-chessboard-square/
// discuss: https://leetcode.com/problems/determine-color-of-a-chessboard-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates
            .as_bytes()
            .into_iter()
            .map(|c| c % 2)
            .sum::<u8>()
            == 1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1812_example_1() {
        let coordinates = "a1".to_string();

        let result = false;

        assert_eq!(Solution::square_is_white(coordinates), result);
    }

    #[test]
    fn test_1812_example_2() {
        let coordinates = "h3".to_string();

        let result = true;

        assert_eq!(Solution::square_is_white(coordinates), result);
    }

    #[test]
    fn test_1812_example_3() {
        let coordinates = "c7".to_string();

        let result = false;

        assert_eq!(Solution::square_is_white(coordinates), result);
    }
}
