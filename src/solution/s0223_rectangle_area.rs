/**
 * [223] Rectangle Area
 *
 * Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.
 * The first rectangle is defined by its bottom-left corner (ax1, ay1) and its top-right corner (ax2, ay2).
 * The second rectangle is defined by its bottom-left corner (bx1, by1) and its top-right corner (bx2, by2).
 *  
 * Example 1:
 * <img alt="Rectangle Area" src="https://assets.leetcode.com/uploads/2021/05/08/rectangle-plane.png" style="width: 700px; height: 365px;" />
 * Input: ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2
 * Output: 45
 *
 * Example 2:
 *
 * Input: ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2
 * Output: 16
 *
 *  
 * Constraints:
 *
 * 	-10^4 <= ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rectangle-area/
// discuss: https://leetcode.com/problems/rectangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let area1 = (ax2 - ax1) * (ay2 - ay1);
        let area2 = (bx2 - bx1) * (by2 - by1);

        if ax2 <= bx1 || ax1 >= bx2 || ay1 >= by2 || ay2 <= by1 {
            area1 + area2
        } else {
            let mut h = vec![ax1, ax2, bx1, bx2];
            let mut v = vec![ay1, ay2, by1, by2];

            h.sort_unstable();
            v.sort_unstable();

            let area3 = (h[2] - h[1]) * (v[2] - v[1]);
            area1 + area2 - area3
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0223_example_1() {
        let ax1 = -3;
        let ay1 = 0;
        let ax2 = 3;
        let ay2 = 4;
        let bx1 = 0;
        let by1 = -1;
        let bx2 = 9;
        let by2 = 2;
        let result = 45;

        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            result
        );
    }

    #[test]
    fn test_0223_example_2() {
        let ax1 = -2;
        let ay1 = -2;
        let ax2 = 2;
        let ay2 = 2;
        let bx1 = -2;
        let by1 = -2;
        let bx2 = 2;
        let by2 = 2;
        let result = 16;

        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            result
        );
    }
}
