/**
 * [218] The Skyline Problem
 *
 * A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.
 * The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:
 *
 * 	lefti is the x coordinate of the left edge of the i^th building.
 * 	righti is the x coordinate of the right edge of the i^th building.
 * 	heighti is the height of the i^th building.
 *
 * You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
 * The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate 0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.
 * Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/merged.jpg" style="width: 800px; height: 331px;" />
 * Input: buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
 * Output: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
 * Explanation:
 * Figure A shows the buildings of the input.
 * Figure B shows the skyline formed by those buildings. The red points in figure B represent the key points in the output list.
 *
 * Example 2:
 *
 * Input: buildings = [[0,2,3],[2,5,3]]
 * Output: [[0,3],[5,0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= buildings.length <= 10^4
 * 	0 <= lefti < righti <= 2^31 - 1
 * 	1 <= heighti <= 2^31 - 1
 * 	buildings is sorted by lefti in non-decreasing order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-skyline-problem/
// discuss: https://leetcode.com/problems/the-skyline-problem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/the-skyline-problem/discuss/954648/Rust-priority-queue-solution
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut bh = std::collections::BinaryHeap::with_capacity(buildings.len());
        let mut points = std::collections::BTreeSet::new();
        for building in buildings.iter() {
            points.insert(building[0]);
            points.insert(building[1]);
        }
        let (mut i, mut h) = (0, 0);
        let mut result = Vec::new();
        for &x in points.iter() {
            while i < buildings.len() && buildings[i][0] == x {
                bh.push((buildings[i][2], buildings[i][0], buildings[i][1]));
                i += 1;
            }
            while let Some(top) = bh.peek() {
                if top.2 <= x {
                    bh.pop();
                } else {
                    break;
                }
            }
            if let Some(top) = bh.peek() {
                if top.0 != h {
                    result.push(vec![x, top.0]);
                    h = top.0;
                }
            } else {
                result.push(vec![x, 0]);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0218_example_1() {
        let buildings = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];
        let result = vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0],
        ];

        assert_eq!(Solution::get_skyline(buildings), result);
    }

    #[test]
    fn test_0218_example_2() {
        let buildings = vec![vec![0, 2, 3], vec![2, 5, 3]];
        let result = vec![vec![0, 3], vec![5, 0]];

        assert_eq!(Solution::get_skyline(buildings), result);
    }
}
