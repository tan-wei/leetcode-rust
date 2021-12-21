/**
 * [0457] Circular Array Loop
 *
 * You are playing a game involving a circular array of non-zero integers nums. Each nums[i] denotes the number of indices forward/backward you must move if you are located at index i:
 *
 * 	If nums[i] is positive, move nums[i] steps forward, and
 * 	If nums[i] is negative, move nums[i] steps backward.
 *
 * Since the array is circular, you may assume that moving forward from the last element puts you on the first element, and moving backwards from the first element puts you on the last element.
 * A cycle in the array consists of a sequence of indices seq of length k where:
 *
 * 	Following the movement rules above results in the repeating index sequence seq[0] -> seq[1] -> ... -> seq[k - 1] -> seq[0] -> ...
 * 	Every nums[seq[j]] is either all positive or all negative.
 * 	k > 1
 *
 * Return true if there is a cycle in nums, or false otherwise.
 *  
 * Example 1:
 *
 * Input: nums = [2,-1,1,2,2]
 * Output: true
 * Explanation:
 * There is a cycle from index 0 -> 2 -> 3 -> 0 -> ...
 * The cycle's length is 3.
 *
 * Example 2:
 *
 * Input: nums = [-1,2]
 * Output: false
 * Explanation:
 * The sequence from index 1 -> 1 -> 1 -> ... is not a cycle because the sequence's length is 1.
 * By definition the sequence's length must be strictly greater than 1 to be a cycle.
 *
 * Example 3:
 *
 * Input: nums = [-2,1,-1,-2,-2]
 * Output: false
 * Explanation:
 * The sequence from index 1 -> 2 -> 1 -> ... is not a cycle because nums[1] is positive, but nums[2] is negative.
 * Every nums[seq[j]] must be either all positive or all negative.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5000
 * 	-1000 <= nums[i] <= 1000
 * 	nums[i] != 0
 *
 *  
 * Follow up: Could you solve it in O(n) time complexity and O(1) extra space complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/circular-array-loop/
// discuss: https://leetcode.com/problems/circular-array-loop/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut nums = nums;
        for i in 0..n {
            if Self::next(&nums, i) == i {
                nums[i] = 0;
            }
        }
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = i;
            while nums[slow] * nums[Self::next(&nums, fast)] > 0
                && nums[slow] * nums[Self::next(&nums, Self::next(&nums, fast))] > 0
            {
                slow = Self::next(&nums, slow);
                fast = Self::next(&nums, Self::next(&nums, fast));
                if slow == fast {
                    return true;
                }
            }
            let mut j = i;
            let val = nums[i];
            while nums[j] * val > 0 {
                let next = Self::next(&nums, j);
                nums[j] = 0;
                j = next;
            }
        }
        false
    }

    fn next(nums: &[i32], index: usize) -> usize {
        let n = nums.len();
        let index = index as i32 + nums[index];
        let index = if index < 0 {
            n as i32 + (index % n as i32)
        } else {
            index % n as i32
        };
        (index as usize) % n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0457_example_1() {
        let nums = vec![2, -1, 1, 2, 2];
        let result = true;

        assert_eq!(Solution::circular_array_loop(nums), result);
    }

    #[test]
    fn test_0457_example_2() {
        let nums = vec![-1, 2];
        let result = false;

        assert_eq!(Solution::circular_array_loop(nums), result);
    }

    #[test]
    fn test_0457_example_3() {
        let nums = vec![-2, 1, -1, -2, -2];
        let result = false;

        assert_eq!(Solution::circular_array_loop(nums), result);
    }
}
