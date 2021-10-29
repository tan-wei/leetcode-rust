/**
 * [0391] Perfect Rectangle
 *
 * Given an array rectangles where rectangles[i] = [xi, yi, ai, bi] represents an axis-aligned rectangle. The bottom-left point of the rectangle is (xi, yi) and the top-right point of it is (ai, bi).
 * Return true if all the rectangles together form an exact cover of a rectangular region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
 * Output: true
 * Explanation: All 5 rectangles together form an exact cover of a rectangular region.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap between the two rectangular regions.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec3-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap in the top center.
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
 * Output: false
 * Explanation: Because two of the rectangles overlap with each other.
 *
 *  
 * Constraints:
 *
 * 	1 <= rectangles.length <= 2 * 10^4
 * 	rectangles[i].length == 4
 * 	-10^5 <= xi, yi, ai, bi <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-rectangle/
// discuss: https://leetcode.com/problems/perfect-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/perfect-rectangle/discuss/867159/Rust-16ms-2mb-Time-O(N)-Space-O(N)
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut corners = std::collections::HashSet::new();
        let mut area = 0;

        for rect in rectangles.iter() {
            let p1 = (rect[0], rect[1]);
            let p2 = (rect[2], rect[1]);
            let p3 = (rect[2], rect[3]);
            let p4 = (rect[0], rect[3]);

            for p in vec![p1, p2, p3, p4] {
                if !corners.insert(p) {
                    corners.remove(&p);
                }
            }

            area += (p3.0 - p1.0) * (p3.1 - p1.1);
        }

        if corners.len() != 4 {
            return false;
        }

        let (mut x1, mut y1, mut x2, mut y2) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for p in corners.iter() {
            x1 = std::cmp::min(p.0, x1);
            y1 = std::cmp::min(p.1, y1);

            x2 = std::cmp::max(p.0, x2);
            y2 = std::cmp::max(p.1, y2);
        }

        area == (x2 - x1) * (y2 - y1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0391_example_1() {
        let rectangles = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4],
        ];
        let result = true;

        assert_eq!(Solution::is_rectangle_cover(rectangles), result);
    }

    #[test]
    fn test_0391_example_2() {
        let rectangles = vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
        ];
        let result = false;

        assert_eq!(Solution::is_rectangle_cover(rectangles), result);
    }

    #[test]
    fn test_0391_example_3() {
        let rectangles = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![3, 2, 4, 4],
        ];
        let result = false;

        assert_eq!(Solution::is_rectangle_cover(rectangles), result);
    }

    #[test]
    fn test_0391_example_4() {
        let rectangles = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4],
        ];
        let result = false;

        assert_eq!(Solution::is_rectangle_cover(rectangles), result);
    }
}
