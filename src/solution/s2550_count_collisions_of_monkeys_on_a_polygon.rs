/**
 * [2550] Count Collisions of Monkeys on a Polygon
 *
 * There is a regular convex polygon with n vertices. The vertices are labeled from 0 to n - 1 in a clockwise direction, and each vertex has exactly one monkey. The following figure shows a convex polygon of 6 vertices.
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/22/hexagon.jpg" style="width: 300px; height: 293px;" />
 * Simultaneously, each monkey moves to a neighboring vertex. A collision happens if at least two monkeys reside on the same vertex after the movement or intersect on an edge.
 * Return the number of ways the monkeys can move so that at least one collision happens. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">n = 3</span>
 * Output: <span class="example-io">6</span>
 * Explanation:
 * There are 8 total possible movements.<br />
 * Two ways such that they collide at some point are:
 *
 * 	Monkey 1 moves in a clockwise direction; monkey 2 moves in an anticlockwise direction; monkey 3 moves in a clockwise direction. Monkeys 1 and 2 collide.
 * 	Monkey 1 moves in an anticlockwise direction; monkey 2 moves in an anticlockwise direction; monkey 3 moves in a clockwise direction. Monkeys 1 and 3 collide.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">n = 4</span>
 * Output: <span class="example-io">14</span>
 * </div>
 *  
 * Constraints:
 *
 * 	3 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/
// discuss: https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2550_example_1() {
        let n = 3;

        let result = 6;

        assert_eq!(Solution::monkey_move(n), result);
    }

    #[test]
    #[ignore]
    fn test_2550_example_2() {
        let n = 4;

        let result = 14;

        assert_eq!(Solution::monkey_move(n), result);
    }
}
