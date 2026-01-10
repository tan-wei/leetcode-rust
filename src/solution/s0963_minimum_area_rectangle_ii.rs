/**
 * [0963] Minimum Area Rectangle II
 *
 * You are given an array of points in the X-Y plane points where points[i] = [xi, yi].
 * Return the minimum area of any rectangle formed from these points, with sides not necessarily parallel to the X and Y axes. If there is not any such rectangle, return 0.
 * Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/21/1a.png" style="width: 398px; height: 400px;" />
 * Input: points = [[1,2],[2,1],[1,0],[0,1]]
 * Output: 2.00000
 * Explanation: The minimum area rectangle occurs at [1,2],[2,1],[1,0],[0,1], with an area of 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/2.png" style="width: 400px; height: 251px;" />
 * Input: points = [[0,1],[2,1],[1,1],[1,0],[2,0]]
 * Output: 1.00000
 * Explanation: The minimum area rectangle occurs at [1,0],[1,1],[2,1],[2,0], with an area of 1.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/3.png" style="width: 383px; height: 400px;" />
 * Input: points = [[0,3],[1,2],[3,1],[1,3],[2,1]]
 * Output: 0
 * Explanation: There is no possible rectangle to form from these points.
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 50
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 4 * 10^4
 * 	All the given points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-area-rectangle-ii/
// discuss: https://leetcode.com/problems/minimum-area-rectangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

type Cross = (i32, i32, i32);
type Point = (i32, i32);

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let mut hm: std::collections::HashMap<Cross, Vec<Point>> = std::collections::HashMap::new();
        let n = points.len();
        let mut res = std::f64::MAX;
        for i in 0..n {
            for j in i + 1..n {
                let x1 = points[i][0];
                let x2 = points[j][0];
                let y1 = points[i][1];
                let y2 = points[j][1];
                let c = (
                    x1 + x2,
                    y1 + y2,
                    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1),
                );
                let points = hm.entry(c).or_default();
                for point in points.iter() {
                    let e1 = Self::edge(x1, y1, point.0, point.1);
                    let e2 = Self::edge(x2, y2, point.0, point.1);
                    res = res.min(e1 * e2);
                }
                points.push((x1, y1));
            }
        }
        if res == std::f64::MAX { 0.0 } else { res }
    }

    fn edge(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        (((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f64).sqrt()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0963_example_1() {
        let points = vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]];
        let result = 2.00000;

        assert_f64_near!(Solution::min_area_free_rect(points), result);
    }

    #[test]
    fn test_0963_example_2() {
        let points = vec![vec![0, 1], vec![2, 1], vec![1, 1], vec![1, 0], vec![2, 0]];
        let result = 1.00000;

        assert_f64_near!(Solution::min_area_free_rect(points), result);
    }

    #[test]
    fn test_0963_example_3() {
        let points = vec![vec![0, 3], vec![1, 2], vec![3, 1], vec![1, 3], vec![2, 1]];
        let result = 0.00000;

        assert_f64_near!(Solution::min_area_free_rect(points), result);
    }
}
