/**
 * [0780] Reaching Points
 *
 * Given four integers sx, sy, tx, and ty, return true if it is possible to convert the point (sx, sy) to the point (tx, ty) through some operations, or false otherwise.
 * The allowed operation on some point (x, y) is to convert it to either (x, x + y) or (x + y, y).
 *  
 * Example 1:
 *
 * Input: sx = 1, sy = 1, tx = 3, ty = 5
 * Output: true
 * Explanation:
 * One series of moves that transforms the starting point to the target is:
 * (1, 1) -> (1, 2)
 * (1, 2) -> (3, 2)
 * (3, 2) -> (3, 5)
 *
 * Example 2:
 *
 * Input: sx = 1, sy = 1, tx = 2, ty = 2
 * Output: false
 *
 * Example 3:
 *
 * Input: sx = 1, sy = 1, tx = 1, ty = 1
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= sx, sy, tx, ty <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reaching-points/
// discuss: https://leetcode.com/problems/reaching-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        if tx == ty {
            return sx == tx && sy == ty;
        }

        let mut tx = tx;
        let mut ty = ty;

        while tx >= 1 && ty >= 1 && tx != ty {
            if tx > ty {
                tx -= std::cmp::max(1, ((tx - sx) / ty)) * ty;
            } else if ty > tx {
                ty -= std::cmp::max(1, ((ty - sy) / tx)) * tx;
            }
            if tx == sx && ty == sy {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0780_example_1() {
        let sx = 1;
        let sy = 1;
        let tx = 3;
        let ty = 5;
        let result = true;

        assert_eq!(Solution::reaching_points(sx, sy, tx, ty), result);
    }

    #[test]
    fn test_0780_example_2() {
        let sx = 1;
        let sy = 1;
        let tx = 2;
        let ty = 2;
        let result = false;

        assert_eq!(Solution::reaching_points(sx, sy, tx, ty), result);
    }

    #[test]
    fn test_0780_example_3() {
        let sx = 1;
        let sy = 1;
        let tx = 1;
        let ty = 1;
        let result = true;

        assert_eq!(Solution::reaching_points(sx, sy, tx, ty), result);
    }
}
