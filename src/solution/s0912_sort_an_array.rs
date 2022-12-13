/**
 * [0912] Sort an Array
 *
 * Given an array of integers nums, sort the array in ascending order and return it.
 * You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
 *  
 * Example 1:
 *
 * Input: nums = [5,2,3,1]
 * Output: [1,2,3,5]
 * Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
 *
 * Example 2:
 *
 * Input: nums = [5,1,1,2,0,0]
 * Output: [0,0,1,1,2,5]
 * Explanation: Note that the values of nums are not necessairly unique.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	-5 * 10^4 <= nums[i] <= 5 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-an-array/
// discuss: https://leetcode.com/problems/sort-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        Self::quicksort(&mut nums, 0, n);
        nums
    }

    fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
        let hi = end - 1;
        let pivot_index = start + (hi - start) / 2;
        nums.swap(pivot_index, hi);
        let pivot = nums[hi];
        let mut i = start;
        let mut j = hi - 1;
        loop {
            while nums[i] < pivot {
                i += 1;
            }

            while j > i && nums[j] >= pivot {
                j -= 1;
            }

            if i >= j {
                nums.swap(i, hi);
                return i;
            }

            nums.swap(i, j);
        }
    }

    fn quicksort(nums: &mut Vec<i32>, start: usize, end: usize) {
        if end - start > 1 {
            let pivot_index = Self::partition(nums, start, end);
            Self::quicksort(nums, start, pivot_index);
            Self::quicksort(nums, pivot_index + 1, end);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0912_example_1() {
        let nums = vec![5, 2, 3, 1];
        let result = vec![1, 2, 3, 5];

        assert_eq!(Solution::sort_array(nums), result);
    }

    #[test]
    fn test_0912_example_2() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let result = vec![0, 0, 1, 1, 2, 5];

        assert_eq!(Solution::sort_array(nums), result);
    }
}
