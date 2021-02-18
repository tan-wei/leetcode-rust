/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" />
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
 *
 * Example 2:
 *
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	n == height.length
 * 	0 <= n <= 3 * 10^4
 * 	0 <= height[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let (mut l_max, mut r_max) = (0, 0);
        let (mut l, mut r) = (0, height.len() - 1);
        let mut result = 0;
        while l < r {
            if height[l] < height[r] {
                l_max = std::cmp::max(l_max, height[l]);
                result += l_max - height[l];
                l += 1;
            } else {
                r_max = std::cmp::max(r_max, height[r]);
                result += r_max - height[r];
                r -= 1;
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
    fn test_0042_example_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn test_0042_example_2() {
        let height = vec![4, 2, 0, 3, 2, 5];

        assert_eq!(Solution::trap(height), 9);
    }
}
