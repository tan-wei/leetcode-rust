/**
 * [11] Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of the line i is at (i, ai) and (i, 0). Find two lines, which, together with the x-axis forms a container, such that the container contains the most water.
 * Notice that you may not slant the container.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
 *
 * Example 2:
 *
 * Input: height = [1,1]
 * Output: 1
 *
 * Example 3:
 *
 * Input: height = [4,3,2,1,4]
 * Output: 16
 *
 * Example 4:
 *
 * Input: height = [1,2,1]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	n == height.length
 * 	2 <= n <= 3 * 10^4
 * 	0 <= height[i] <= 3 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/
// discuss: https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut i = 0;
        let mut j = height.len() - 1;

        while i < j {
            let area = (j - i) as i32 * height[i].min(height[j]);

            result = result.max(area);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
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
    fn test_0011_example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test_0011_example_2() {
        let height = vec![1, 1];

        assert_eq!(Solution::max_area(height), 1);
    }

    #[test]
    fn test_0011_example_3() {
        let height = vec![4, 3, 2, 1, 4];

        assert_eq!(Solution::max_area(height), 16);
    }

    #[test]
    fn test_0011_example_4() {
        let height = vec![1, 2, 1];

        assert_eq!(Solution::max_area(height), 2);
    }
}
