/**
 * [0497] Random Point in Non-overlapping Rectangles
 *
 * You are given an array of non-overlapping axis-aligned rectangles rects where rects[i] = [ai, bi, xi, yi] indicates that (ai, bi) is the bottom-left corner point of the i^th rectangle and (xi, yi) is the top-right corner point of the i^th rectangle. Design an algorithm to pick a random integer point inside the space covered by one of the given rectangles. A point on the perimeter of a rectangle is included in the space covered by the rectangle.
 * Any integer point inside the space covered by one of the given rectangles should be equally likely to be returned.
 * Note that an integer point is a point that has integer coordinates.
 * Implement the Solution class:
 *
 * 	Solution(int[][] rects) Initializes the object with the given rectangles rects.
 * 	int[] pick() Returns a random integer point [u, v] inside the space covered by one of the given rectangles.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/24/lc-pickrandomrec.jpg" style="width: 419px; height: 539px;" />
 * Input
 * ["Solution", "pick", "pick", "pick", "pick", "pick"]
 * [[[[-2, -2, 1, 1], [2, 2, 4, 6]]], [], [], [], [], []]
 * Output
 * [null, [1, -2], [1, -1], [-1, -2], [-2, -2], [0, 0]]
 * Explanation
 * Solution solution = new Solution([[-2, -2, 1, 1], [2, 2, 4, 6]]);
 * solution.pick(); // return [1, -2]
 * solution.pick(); // return [1, -1]
 * solution.pick(); // return [-1, -2]
 * solution.pick(); // return [-2, -2]
 * solution.pick(); // return [0, 0]
 *
 *  
 * Constraints:
 *
 * 	1 <= rects.length <= 100
 * 	rects[i].length == 4
 * 	-10^9 <= ai < xi <= 10^9
 * 	-10^9 <= bi < yi <= 10^9
 * 	xi - ai <= 2000
 * 	yi - bi <= 2000
 * 	All the rectangles do not overlap.
 * 	At most 10^4 calls will be made to pick.
 *
 */
// problem: https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/
// discuss: https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use rand::{thread_rng, Rng};

struct Solution {
    rects: Vec<Vec<i32>>,
    pref: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let pref = rects
            .iter()
            .scan(0, |state, rect| {
                *state += (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
                Some(*state)
            })
            .collect();

        Self { rects, pref }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        let value: i32 = rng.gen_range(1, self.pref.last().unwrap() + 1);

        let index = match self.pref.binary_search(&value) {
            Ok(val) => val,
            Err(val) => val,
        };
        let offset = if index == 0 {
            1
        } else {
            self.pref[index - 1] + 1
        };

        let width = self.rects[index][2] - self.rects[index][0] + 1;
        let value = value - offset;
        vec![
            value % width + self.rects[index][0],
            value / width + self.rects[index][1],
        ]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0497_example_1() {
        let mut solution = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
        solution.pick();
        solution.pick();
        solution.pick();
        solution.pick();
        solution.pick();
    }
}
