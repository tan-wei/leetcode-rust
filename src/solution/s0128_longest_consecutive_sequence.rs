/**
 * [128] Longest Consecutive Sequence
 *
 * Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
 *  
 * Example 1:
 *
 * Input: nums = [100,4,200,1,3,2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
 *
 * Example 2:
 *
 * Input: nums = [0,3,7,2,5,8,4,6,0,1]
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 *  
 * Follow up: Could you implement the O(n) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-consecutive-sequence/
// discuss: https://leetcode.com/problems/longest-consecutive-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums_set = std::collections::HashSet::new();

        for num in nums {
            nums_set.insert(num);
        }

        let mut longest = 0;

        for num in nums_set.iter() {
            if !nums_set.contains(&(*num - 1)) {
                let mut current = 1;
                let mut current_num = *num;
                while nums_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current += 1;
                }
                longest = longest.max(current);
            }
        }

        longest
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0128_example_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let result = 4;

        assert_eq!(Solution::longest_consecutive(nums), result);
    }

    #[test]
    fn test_0128_example_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = 9;

        assert_eq!(Solution::longest_consecutive(nums), result);
    }
}
