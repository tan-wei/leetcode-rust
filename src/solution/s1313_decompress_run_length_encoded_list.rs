/**
 * [1313] Decompress Run-Length Encoded List
 *
 * We are given a list nums of integers representing a list compressed with run-length encoding.
 * Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).  For each such pair, there are freq elements with value val concatenated in a sublist. Concatenate all the sublists from left to right to generate the decompressed list.
 * Return the decompressed list.
 *
 * Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: [2,4,4,4]
 * Explanation: The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
 * The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
 * At the end the concatenation [2] + [4,4,4] is [2,4,4,4].
 *
 * Example 2:
 *
 * Input: nums = [1,1,2,3]
 * Output: [1,3,3]
 *
 *
 * Constraints:
 *
 * 	2 <= nums.length <= 100
 * 	nums.length % 2 == 0
 * 	<font face="monospace">1 <= nums[i] <= 100</font>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decompress-run-length-encoded-list/
// discuss: https://leetcode.com/problems/decompress-run-length-encoded-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|c| [c[1]].repeat(c[0] as usize))
            .collect::<Vec<i32>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1313_example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = vec![2, 4, 4, 4];

        assert_eq!(Solution::decompress_rl_elist(nums), result);
    }

    #[test]
    fn test_1313_example_2() {
        let nums = vec![1, 1, 2, 3];
        let result = vec![1, 3, 3];

        assert_eq!(Solution::decompress_rl_elist(nums), result);
    }
}
