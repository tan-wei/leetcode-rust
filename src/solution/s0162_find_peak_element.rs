/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is strictly greater than its neighbors.
 * Given an integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 * You must write an algorithm that runs in O(log n) time.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 * Example 2:
 *
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 5
 * Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	nums[i] != nums[i + 1] for all valid i.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-peak-element/
// discuss: https://leetcode.com/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = (low + (high - low) / 2) as usize;
            if nums[mid] < nums[mid + 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0162_example_1() {
        let nums = vec![1, 2, 3, 1];
        let result = 2;

        assert_eq!(Solution::find_peak_element(nums), result);
    }

    #[test]
    fn test_0162_example_2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let result = 5;

        assert_eq!(Solution::find_peak_element(nums), result);
    }
}
