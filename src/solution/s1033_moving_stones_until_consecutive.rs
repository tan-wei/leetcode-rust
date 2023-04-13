/**
 * [1033] Moving Stones Until Consecutive
 *
 * There are three stones in different positions on the X-axis. You are given three integers a, b, and c, the positions of the stones.
 * In one move, you pick up a stone at an endpoint (i.e., either the lowest or highest position stone), and move it to an unoccupied position between those endpoints. Formally, let's say the stones are currently at positions x, y, and z with x < y < z. You pick up the stone at either position x or position z, and move that stone to an integer position k, with x < k < z and k != y.
 * The game ends when you cannot make any more moves (i.e., the stones are in three consecutive positions).
 * Return an integer array answer of length 2 where:
 *
 * 	answer[0] is the minimum number of moves you can play, and
 * 	answer[1] is the maximum number of moves you can play.
 *
 *  
 * Example 1:
 *
 * Input: a = 1, b = 2, c = 5
 * Output: [1,2]
 * Explanation: Move the stone from 5 to 3, or move the stone from 5 to 4 to 3.
 *
 * Example 2:
 *
 * Input: a = 4, b = 3, c = 2
 * Output: [0,0]
 * Explanation: We cannot make any moves.
 *
 * Example 3:
 *
 * Input: a = 3, b = 5, c = 1
 * Output: [1,2]
 * Explanation: Move the stone from 1 to 4; or move the stone from 1 to 2 to 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= a, b, c <= 100
 * 	a, b, and c have different values.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/moving-stones-until-consecutive/
// discuss: https://leetcode.com/problems/moving-stones-until-consecutive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut sorted = vec![a, b, c];
        sorted.sort();
        let x = sorted[0];
        let y = sorted[1];
        let z = sorted[2];

        let (min, max) = match (y - x, z - y) {
            (1, 1) => (0, 0),
            (1, _) | (_, 1) | (2, _) | (_, 2) => (1, z - x - 2),
            _ => (2, z - x - 2),
        };

        vec![min, max]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1033_example_1() {
        let a = 1;
        let b = 2;
        let c = 5;
        let result = vec![1, 2];

        assert_eq!(Solution::num_moves_stones(a, b, c), result);
    }

    #[test]
    fn test_1033_example_2() {
        let a = 4;
        let b = 3;
        let c = 2;
        let result = vec![0, 0];

        assert_eq!(Solution::num_moves_stones(a, b, c), result);
    }

    #[test]
    fn test_1033_example_3() {
        let a = 3;
        let b = 5;
        let c = 1;
        let result = vec![1, 2];

        assert_eq!(Solution::num_moves_stones(a, b, c), result);
    }
}
