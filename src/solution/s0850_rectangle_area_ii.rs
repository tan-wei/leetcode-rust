/**
 * [0850] Rectangle Area II
 *
 * You are given a 2D array of axis-aligned rectangles. Each rectangle[i] = [xi1, yi1, xi2, yi2] denotes the i^th rectangle where (xi1, yi1) are the coordinates of the bottom-left corner, and (xi2, yi2) are the coordinates of the top-right corner.
 * Calculate the total area covered by all rectangles in the plane. Any area covered by two or more rectangles should only be counted once.
 * Return the total area. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/06/rectangle_area_ii_pic.png" style="width: 600px; height: 450px;" />
 * Input: rectangles = [[0,0,2,2],[1,0,2,3],[1,0,3,1]]
 * Output: 6
 * Explanation: A total area of 6 is covered by all three rectangles, as illustrated in the picture.
 * From (1,1) to (2,2), the green and red rectangles overlap.
 * From (1,0) to (2,3), all three rectangles overlap.
 *
 * <strong class="example">Example 2:
 *
 * Input: rectangles = [[0,0,1000000000,1000000000]]
 * Output: 49
 * Explanation: The answer is 10^18 modulo (10^9 + 7), which is 49.
 *
 *  
 * Constraints:
 *
 * 	1 <= rectangles.length <= 200
 * 	rectanges[i].length == 4
 * 	0 <= xi1, yi1, xi2, yi2 <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rectangle-area-ii/
// discuss: https://leetcode.com/problems/rectangle-area-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i32 = 1000000007;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut xs = std::collections::BTreeSet::new();
        let mut ys = std::collections::BTreeSet::new();
        for rec in rectangles.iter() {
            xs.insert(rec[0]);
            xs.insert(rec[2]);
            ys.insert(rec[1]);
            ys.insert(rec[3]);
        }
        // BTreeSet makes sure that all values are sorted
        let xs: Vec<i32> = xs.iter().map(|x| *x).collect();
        let ys: Vec<i32> = ys.iter().map(|y| *y).collect();

        let mut canvas = vec![vec![false; ys.len()]; xs.len()];
        let mut area: i32 = 0;

        for rec in rectangles.iter() {
            let mut xi = xs.binary_search(&rec[0]).unwrap();
            for xi in (xi..).take_while(|xi| xs[*xi] != rec[2]) {
                let mut yi = ys.binary_search(&rec[1]).unwrap();
                for yi in (yi..).take_while(|yi| ys[*yi] != rec[3]) {
                    if canvas[xi][yi] {
                        // the pixel is already painted
                        continue;
                    }
                    canvas[xi][yi] = true;
                    let rec_area = (xs[xi + 1] as i64 - xs[xi] as i64)
                        * (ys[yi + 1] as i64 - ys[yi] as i64)
                        % MOD as i64;
                    area = (area + rec_area as i32) % MOD;
                }
            }
        }

        area
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0850_example_1() {
        let rectangles = vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]];
        let result = 6;

        assert_eq!(Solution::rectangle_area(rectangles), result);
    }

    #[test]
    fn test_0850_example_2() {
        let rectangles = vec![vec![0, 0, 1000000000, 1000000000]];
        let result = 49;

        assert_eq!(Solution::rectangle_area(rectangles), result);
    }
}
