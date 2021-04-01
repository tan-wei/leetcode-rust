/**
 * [84] Largest Rectangle in Histogram
 *
 * Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg" style="width: 522px; height: 242px;" />
 * Input: heights = [2,1,5,6,2,3]
 * Output: 10
 * Explanation: The above is a histogram where width of each bar is 1.
 * The largest rectangle is shown in the red area, which has an area = 10 units.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg" style="width: 202px; height: 362px;" />
 * Input: heights = [2,4]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= heights.length <= 10^5
 * 	0 <= heights[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-rectangle-in-histogram/
// discuss: https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        let mut res: i32 = 0;
        let mut stack: Vec<usize> = Vec::new();

        heights.push(0);
        heights.insert(0, 0);

        for (i, h) in heights.iter().enumerate() {
            while stack.len() > 0 && heights[*stack.iter().last().unwrap()] > *h {
                let j = stack.pop().unwrap();
                let width = (i - stack[stack.len() - 1] - 1) as i32;
                let size = width * heights[j];
                res = res.max(size);
            }
            stack.push(i);
        }
        return res;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0084_example_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let result = 10;

        assert_eq!(Solution::largest_rectangle_area(heights), result);
    }

    #[test]
    fn test_0084_example_2() {
        let heights = vec![2, 4];
        let result = 4;

        assert_eq!(Solution::largest_rectangle_area(heights), result);
    }
}
