/**
 * [27] Remove Element
 *
 * Given an array nums and a value val, remove all instances of that value <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and return the new length.
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * The order of elements can be changed. It doesn't matter what you leave beyond the new length.
 * Clarification:
 * Confused why the returned value is an integer but your answer is an array?
 * Note that the input array is passed in by reference, which means a modification to the input array will be known to the caller as well.
 * Internally you can think of this:
 *
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeElement(nums, val);
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 *  
 * Example 1:
 *
 * Input: nums = [3,2,2,3], val = 3
 * Output: 2, nums = [2,2]
 * Explanation: Your function should return length = 2, with the first two elements of nums being 2.
 * It doesn't matter what you leave beyond the returned length. For example if you return 2 with nums = [2,2,3,3] or nums = [2,2,0,0], your answer will be accepted.
 *
 * Example 2:
 *
 * Input: nums = [0,1,2,2,3,0,4,2], val = 2
 * Output: 5, nums = [0,1,4,0,3]
 * Explanation: Your function should return length = 5, with the first five elements of nums containing 0, 1, 3, 0, and 4. Note that the order of those five elements can be arbitrary. It doesn't matter what values are set beyond the returned length.
 *
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 100
 * 	0 <= nums[i] <= 50
 * 	0 <= val <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-element/
// discuss: https://leetcode.com/problems/remove-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }

    pub fn remove_element_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut current = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[current] = nums[i];
                current += 1
            }
        }

        current as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0027_example_1() {
        // Version 1
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        assert_eq_sorted!(nums[0..2].to_vec(), vec![2, 2]);

        // Version 2
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        assert_eq!(Solution::remove_element_v2(&mut nums, val), 2);
        assert_eq_sorted!(nums[0..2].to_vec(), vec![2, 2]);
    }

    #[test]
    fn test_0027_example_2() {
        // Version 1
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        assert_eq!(Solution::remove_element(&mut nums, val), 5);
        assert_eq_sorted!(nums[0..5].to_vec(), vec![0, 1, 4, 0, 3]);

        // Version 2
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        assert_eq!(Solution::remove_element_v2(&mut nums, val), 5);
        assert_eq_sorted!(nums[0..5].to_vec(), vec![0, 1, 4, 0, 3]);
    }
}
