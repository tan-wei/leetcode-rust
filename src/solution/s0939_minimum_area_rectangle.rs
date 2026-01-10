/**
 * [0939] Minimum Area Rectangle
 *
 * You are given an array of points in the X-Y plane points where points[i] = [xi, yi].
 * Return the minimum area of a rectangle formed from these points, with sides parallel to the X and Y axes. If there is not any such rectangle, return 0.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/rec1.JPG" style="width: 500px; height: 447px;" />
 * Input: points = [[1,1],[1,3],[3,1],[3,3],[2,2]]
 * Output: 4
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/rec2.JPG" style="width: 500px; height: 477px;" />
 * Input: points = [[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 500
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 4 * 10^4
 * 	All the given points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-area-rectangle/
// discuss: https://leetcode.com/problems/minimum-area-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut hs = std::collections::HashSet::new();
        for i in 0..n {
            hs.insert((points[i][0], points[i][1]));
        }
        let mut min = i32::MAX;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];
                if x2 != x1 && y2 != y1 && hs.contains(&(x1, y2)) && hs.contains(&(x2, y1)) {
                    min = min.min((x2 - x1).abs() * (y2 - y1).abs())
                }
            }
        }
        if min == i32::MAX { 0 } else { min }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0939_example_1() {
        let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]];
        let result = 4;

        assert_eq!(Solution::min_area_rect(points), result);
    }

    #[test]
    fn test_0939_example_2() {
        let points = vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3],
        ];
        let result = 2;

        assert_eq!(Solution::min_area_rect(points), result);
    }
}
