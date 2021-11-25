/**
 * [0421] Maximum XOR of Two Numbers in an Array
 *
 * Given an integer array nums, return the maximum result of nums[i] XOR nums[j], where 0 <= i <= j < n.
 *  
 * Example 1:
 *
 * Input: nums = [3,10,5,25,2,8]
 * Output: 28
 * Explanation: The maximum result is 5 XOR 25 = 28.
 * Example 2:
 *
 * Input: nums = [0]
 * Output: 0
 *
 * Example 3:
 *
 * Input: nums = [2,4]
 * Output: 6
 *
 * Example 4:
 *
 * Input: nums = [8,10,2]
 * Output: 10
 *
 * Example 5:
 *
 * Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
 * Output: 127
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^5
 * 	0 <= nums[i] <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
// discuss: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/804241/Rust-cheapest-and-best
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        (0..=31)
            .rev()
            .scan(0, |mask, i| {
                *mask = *mask | 1 << i;
                Some((*mask, 1 << i))
            })
            .fold(0, |result, (mask, bit)| {
                let greedy = result | bit;
                let s = nums
                    .iter()
                    .map(|n| n & mask)
                    .collect::<std::collections::HashSet<i32>>();
                s.iter()
                    .find(|&left_part| s.contains(&(left_part ^ greedy)))
                    .map(|_| greedy)
                    .unwrap_or(result)
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0421_example_1() {
        let nums = vec![3, 10, 5, 25, 2, 8];
    }
}
