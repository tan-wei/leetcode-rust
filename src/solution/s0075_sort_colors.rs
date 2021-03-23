/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 *  
 * Example 1:
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 * Example 2:
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 * Example 3:
 * Input: nums = [0]
 * Output: [0]
 * Example 4:
 * Input: nums = [1]
 * Output: [1]
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 300
 * 	nums[i] is 0, 1, or 2.
 *
 *  
 * Follow up:
 *
 * 	Could you solve this problem without using the library's sort function?
 * 	Could you come up with a one-pass algorithm using only O(1) constant space?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    const RED: i32 = 0;
    const WHITE: i32 = 1;
    const BLUE: i32 = 2;

    pub fn sort_colors(nums: &mut Vec<i32>) {
        // Do Dutch partitioning
        let (mut red, mut white, mut blue) = (0, 0, nums.len() - 1);

        while white <= blue {
            if nums[white] == Self::RED {
                nums.swap(white, red);
                white += 1;
                red += 1;
            } else if nums[white] == Self::WHITE {
                white += 1;
            } else {
                // nums[white] == BLUE
                nums.swap(white, blue);
                if blue > 0 {
                    // To avoid any runtime error.
                    blue -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0075_example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let result = vec![0, 0, 1, 1, 2, 2];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0075_example_2() {
        let mut nums = vec![2, 0, 1];
        let result = vec![0, 1, 2];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0075_example_3() {
        let mut nums = vec![0];
        let result = vec![0];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    fn test_0075_example_4() {
        let mut nums = vec![1];
        let result = vec![1];

        Solution::sort_colors(&mut nums);

        assert_eq!(nums, result);
    }
}
