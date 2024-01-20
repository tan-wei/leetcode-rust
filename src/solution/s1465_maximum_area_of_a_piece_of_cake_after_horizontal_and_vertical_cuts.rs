/**
 * [1465] Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
 *
 * You are given a rectangular cake of size h x w and two arrays of integers horizontalCuts and verticalCuts where:
 *
 * 	horizontalCuts[i] is the distance from the top of the rectangular cake to the i^th horizontal cut and similarly, and
 * 	verticalCuts[j] is the distance from the left of the rectangular cake to the j^th vertical cut.
 *
 * Return the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays horizontalCuts and verticalCuts. Since the answer can be a large number, return this modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_2.png" style="width: 225px; height: 240px;" />
 * Input: h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
 * Output: 4
 * Explanation: The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green piece of cake has the maximum area.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_3.png" style="width: 225px; height: 240px;" />
 * Input: h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
 * Output: 6
 * Explanation: The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green and yellow pieces of cake have the maximum area.
 *
 * Example 3:
 *
 * Input: h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	2 <= h, w <= 10^9
 * 	1 <= horizontalCuts.length <= min(h - 1, 10^5)
 * 	1 <= verticalCuts.length <= min(w - 1, 10^5)
 * 	1 <= horizontalCuts[i] < h
 * 	1 <= verticalCuts[i] < w
 * 	All the elements in horizontalCuts are distinct.
 * 	All the elements in verticalCuts are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
// discuss: https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        let mut vertical_cuts = vertical_cuts;

        horizontal_cuts.extend(&[0, h]);
        vertical_cuts.extend(&[0, w]);

        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();

        let hmax = horizontal_cuts
            .windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .unwrap() as i64;
        let vmax = vertical_cuts.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;

        (hmax * vmax % 1_000_000_007) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1465_example_1() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];

        let result = 4;

        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts, vertical_cuts),
            result
        );
    }

    #[test]
    fn test_1465_example_2() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];

        let result = 6;

        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts, vertical_cuts),
            result
        );
    }

    #[test]
    fn test_1465_example_3() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3];
        let vertical_cuts = vec![3];

        let result = 9;

        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts, vertical_cuts),
            result
        );
    }
}
