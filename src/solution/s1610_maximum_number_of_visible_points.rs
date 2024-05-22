/**
 * [1610] Maximum Number of Visible Points
 *
 * You are given an array points, an integer angle, and your location, where location = [posx, posy] and points[i] = [xi, yi] both denote integral coordinates on the X-Y plane.
 * Initially, you are facing directly east from your position. You cannot move from your position, but you can rotate. In other words, posx and posy cannot be changed. Your field of view in degrees is represented by angle, determining how wide you can see from any given view direction. Let d be the amount in degrees that you rotate counterclockwise. Then, your field of view is the inclusive range of angles [d - angle/2, d + angle/2].
 *
 * <video autoplay="" controls="" height="360" muted="" style="max-width:100%;height:auto;" width="480"><source src="https://assets.leetcode.com/uploads/2020/09/30/angle.mp4" type="video/mp4" />Your browser does not support the video tag or this video format.</video>
 *
 * You can see some set of points if, for each point, the angle formed by the point, your position, and the immediate east direction from your position is in your field of view.
 * There can be multiple points at one coordinate. There may be points at your location, and you can always see these points regardless of your rotation. Points do not obstruct your vision to other points.
 * Return the maximum number of points you can see.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/30/89a07e9b-00ab-4967-976a-c723b2aa8656.png" style="width: 400px; height: 300px;" />
 * Input: points = [[2,1],[2,2],[3,3]], angle = 90, location = [1,1]
 * Output: 3
 * Explanation: The shaded region represents your field of view. All points can be made visible in your field of view, including [3,3] even though [2,2] is in front and in the same line of sight.
 *
 * Example 2:
 *
 * Input: points = [[2,1],[2,2],[3,4],[1,1]], angle = 90, location = [1,1]
 * Output: 4
 * Explanation: All points can be made visible in your field of view, including the one at your location.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/30/5010bfd3-86e6-465f-ac64-e9df941d2e49.png" style="width: 690px; height: 348px;" />
 * Input: points = [[1,0],[2,1]], angle = 13, location = [1,1]
 * Output: 1
 * Explanation: You can only see one of the two points, as shown above.
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 10^5
 * 	points[i].length == 2
 * 	location.length == 2
 * 	0 <= angle < 360
 * 	0 <= posx, posy, xi, yi <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-visible-points/
// discuss: https://leetcode.com/problems/maximum-number-of-visible-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        let mut extra = 0;
        let mut angles = points
            .iter()
            .filter(|p| {
                if p[0] == location[0] && p[1] == location[1] {
                    extra += 1;
                    false
                } else {
                    true
                }
            })
            .map(|p| ((location[1] - p[1]) as f64).atan2((location[0] - p[0]) as f64))
            .map(|a| {
                360.0
                    * (if a < 0.0 {
                        a + 2.0 * std::f64::consts::PI
                    } else {
                        a
                    })
                    / (2.0 * std::f64::consts::PI)
            })
            .collect::<Vec<_>>();

        for i in 0..angles.len() {
            angles.push(angles[i] + 360.0);
        }

        if angles.len() == 0 {
            return extra as i32;
        }

        angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let angle = angle as f64;
        let mut best = 1;
        let mut l = 0;
        let mut r = 1;

        while r < angles.len() && l < angles.len() {
            while r < angles.len() && angles[r] - angles[l] <= angle {
                r += 1;
            }
            best = best.max(r - l);
            l += 1;
        }

        (extra + best) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1610_example_1() {
        let points = vec![vec![2, 1], vec![2, 2], vec![3, 3]];
        let angle = 90;
        let location = vec![1, 1];

        let result = 3;

        assert_eq!(Solution::visible_points(points, angle, location), result);
    }

    #[test]
    fn test_1610_example_2() {
        let points = vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]];
        let angle = 90;
        let location = vec![1, 1];

        let result = 4;

        assert_eq!(Solution::visible_points(points, angle, location), result);
    }

    #[test]
    fn test_1610_example_3() {
        let points = vec![vec![1, 0], vec![2, 1]];
        let angle = 13;
        let location = vec![1, 1];

        let result = 1;

        assert_eq!(Solution::visible_points(points, angle, location), result);
    }
}
