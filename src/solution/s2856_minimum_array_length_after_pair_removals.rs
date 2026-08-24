/**
 * [2856] Minimum Array Length After Pair Removals
 *
 * Given an integer array num sorted in non-decreasing order.
 * You can perform the following operation any number of times:
 *
 * 	Choose two indices, i and j, where nums[i] < nums[j].
 * 	Then, remove the elements at indices i and j from nums. The remaining elements retain their original order, and the array is re-indexed.
 *
 * Return the minimum length of nums after applying the operation zero or more times.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,2,3,4]</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * <img src="https://assets.leetcode.com/uploads/2024/05/18/tcase1.gif" style="width: 160px; height: 70px;" />
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1,1,2,2,3,3]</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * <img src="https://assets.leetcode.com/uploads/2024/05/19/tcase2.gif" style="width: 240px; height: 70px;" />
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [1000000000,1000000000]</span>
 * Output: <span class="example-io">2</span>
 * Explanation:
 * Since both numbers are equal, they cannot be removed.
 * </div>
 * Example 4:
 * <div class="example-block">
 * Input: <span class="example-io">nums = [2,3,4,4,4]</span>
 * Output: <span class="example-io">1</span>
 * Explanation:
 * <img src="https://assets.leetcode.com/uploads/2024/05/19/tcase3.gif" style="width: 210px; height: 70px;" />
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 * 	nums is sorted in non-decreasing order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-array-length-after-pair-removals/
// discuss: https://leetcode.com/problems/minimum-array-length-after-pair-removals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2856_example_1() {
        let nums = vec![1, 2, 3, 4];

        let result = 0;

        assert_eq!(Solution::min_length_after_removals(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2856_example_2() {
        let nums = vec![1, 1, 2, 2, 3, 3];

        let result = 0;

        assert_eq!(Solution::min_length_after_removals(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2856_example_3() {
        let nums = vec![1000000000, 1000000000];

        let result = 2;

        assert_eq!(Solution::min_length_after_removals(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2856_example_4() {
        let nums = vec![2, 3, 4, 4, 4];

        let result = 1;

        assert_eq!(Solution::min_length_after_removals(nums), result);
    }
}
