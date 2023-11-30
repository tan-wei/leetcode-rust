/**
 * [1401] Circle and Rectangle Overlapping
 *
 * You are given a circle represented as (radius, xCenter, yCenter) and an axis-aligned rectangle represented as (x1, y1, x2, y2), where (x1, y1) are the coordinates of the bottom-left corner, and (x2, y2) are the coordinates of the top-right corner of the rectangle.
 * Return true if the circle and rectangle are overlapped otherwise return false. In other words, check if there is any point (xi, yi) that belongs to the circle and the rectangle at the same time.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/20/sample_4_1728.png" style="width: 258px; height: 167px;" />
 * Input: radius = 1, xCenter = 0, yCenter = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
 * Output: true
 * Explanation: Circle and rectangle share the point (1,0).
 *
 * Example 2:
 *
 * Input: radius = 1, xCenter = 1, yCenter = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
 * Output: false
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/20/sample_2_1728.png" style="width: 150px; height: 135px;" />
 * Input: radius = 1, xCenter = 0, yCenter = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= radius <= 2000
 * 	-10^4 <= xCenter, yCenter <= 10^4
 * 	-10^4 <= x1 < x2 <= 10^4
 * 	-10^4 <= y1 < y2 <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/circle-and-rectangle-overlapping/
// discuss: https://leetcode.com/problems/circle-and-rectangle-overlapping/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = if x_center < x1 {
            x1
        } else if x_center > x2 {
            x2
        } else {
            x_center
        };

        let y = if y_center < y1 {
            y1
        } else if y_center > y2 {
            y2
        } else {
            y_center
        };

        let dx = x_center - x;
        let dy = y_center - y;

        dx * dx + dy * dy <= radius * radius
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1401_example_1() {
        let radius = 1;
        let x_center = 0;
        let y_center = 0;
        let x1 = 1;
        let y1 = -1;
        let x2 = 3;
        let y2 = 1;

        let result = true;

        assert_eq!(
            Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
            result
        );
    }

    #[test]
    fn test_1401_example_2() {
        let radius = 1;
        let x_center = 1;
        let y_center = 1;
        let x1 = 1;
        let y1 = -3;
        let x2 = 2;
        let y2 = -1;

        let result = false;

        assert_eq!(
            Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
            result
        );
    }

    #[test]
    fn test_1401_example_3() {
        let radius = 1;
        let x_center = 0;
        let y_center = 0;
        let x1 = -1;
        let y1 = 0;
        let x2 = 0;
        let y2 = 1;

        let result = true;

        assert_eq!(
            Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
            result
        );
    }
}
