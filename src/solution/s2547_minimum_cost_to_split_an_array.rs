/**
 * [2547] Minimum Cost to Split an Array
 *
 * You are given an integer array nums and an integer k.
 * Split the array into some number of non-empty subarrays. The cost of a split is the sum of the importance value of each subarray in the split.
 * Let trimmed(subarray) be the version of the subarray where all numbers which appear only once are removed.
 *
 * 	For example, trimmed([3,1,2,4,3,4]) = [3,4,3,4].
 *
 * The importance value of a subarray is k + trimmed(subarray).length.
 *
 * 	For example, if a subarray is [1,2,3,3,3,4,4], then <font face="monospace">trimmed(</font>[1,2,3,3,3,4,4]) = [3,3,3,4,4].The importance value of this subarray will be k + 5.
 *
 * Return the minimum possible cost of a split of nums.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,2,1,3,3], k = 2
 * Output: 8
 * Explanation: We split nums to have two subarrays: [1,2], [1,2,1,3,3].
 * The importance value of [1,2] is 2 + (0) = 2.
 * The importance value of [1,2,1,3,3] is 2 + (2 + 2) = 6.
 * The cost of the split is 2 + 6 = 8. It can be shown that this is the minimum possible cost among all the possible splits.
 *
 * Example 2:
 *
 * Input: nums = [1,2,1,2,1], k = 2
 * Output: 6
 * Explanation: We split nums to have two subarrays: [1,2], [1,2,1].
 * The importance value of [1,2] is 2 + (0) = 2.
 * The importance value of [1,2,1] is 2 + (2) = 4.
 * The cost of the split is 2 + 4 = 6. It can be shown that this is the minimum possible cost among all the possible splits.
 *
 * Example 3:
 *
 * Input: nums = [1,2,1,2,1], k = 5
 * Output: 10
 * Explanation: We split nums to have one subarray: [1,2,1,2,1].
 * The importance value of [1,2,1,2,1] is 5 + (3 + 2) = 10.
 * The cost of the split is 10. It can be shown that this is the minimum possible cost among all the possible splits.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] < nums.length
 * 	1 <= k <= 10^9
 *
 *  
 * <style type="text/css">.spoilerbutton {display:block; border:dashed; padding: 0px 0px; margin:10px 0px; font-size:150%; font-weight: bold; color:#000000; background-color:cyan; outline:0;
 * }
 * .spoiler {overflow:hidden;}
 * .spoiler > div {-webkit-transition: all 0s ease;-moz-transition: margin 0s ease;-o-transition: all 0s ease;transition: margin 0s ease;}
 * .spoilerbutton[value="Show Message"] + .spoiler > div {margin-top:-500%;}
 * .spoilerbutton[value="Hide Message"] + .spoiler {padding:5px;}
 * </style>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-split-an-array/
// discuss: https://leetcode.com/problems/minimum-cost-to-split-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2547_example_1() {
        let nums = vec![1, 2, 1, 2, 1, 3, 3];
        let k = 2;

        let result = 8;

        assert_eq!(Solution::min_cost(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2547_example_2() {
        let nums = vec![1, 2, 1, 2, 1];
        let k = 2;

        let result = 6;

        assert_eq!(Solution::min_cost(nums, k), result);
    }

    #[test]
    #[ignore]
    fn test_2547_example_3() {
        let nums = vec![1, 2, 1, 2, 1];
        let k = 5;

        let result = 10;

        assert_eq!(Solution::min_cost(nums, k), result);
    }
}
