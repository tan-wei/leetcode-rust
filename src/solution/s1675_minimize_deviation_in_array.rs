/**
 * [1675] Minimize Deviation in Array
 *
 * You are given an array nums of n positive integers.
 * You can perform two types of operations on any element of the array any number of times:
 *
 * 	If the element is even, divide it by 2.
 *
 * 		For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,<u>2</u>].
 *
 *
 * 	If the element is odd, multiply it by 2.
 *
 * 		For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [<u>2</u>,2,3,4].
 *
 *
 *
 * The deviation of the array is the maximum difference between any two elements in the array.
 * Return the minimum deviation the array can have after performing some number of operations.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: 1
 * Explanation: You can transform the array to [1,2,3,<u>2</u>], then to [<u>2</u>,2,3,2], then the deviation will be 3 - 2 = 1.
 *
 * Example 2:
 *
 * Input: nums = [4,1,5,20,3]
 * Output: 3
 * Explanation: You can transform the array after two operations to [4,<u>2</u>,5,<u>5</u>,3], then the deviation will be 5 - 2 = 3.
 *
 * Example 3:
 *
 * Input: nums = [2,10,8]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	2 <= n <= 5 * 10^<span style="font-size: 10.8333px;">4</span>
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-deviation-in-array/
// discuss: https://leetcode.com/problems/minimize-deviation-in-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimize-deviation-in-array/solutions/3188337/just-a-runnable-solution/
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut min = std::i32::MAX;
        for num in nums.iter_mut() {
            if *num % 2 == 1 {
                *num *= 2;
            }
            min = min.min(*num);
        }
        let mut result = std::i32::MAX;
        let mut heap = std::collections::BinaryHeap::new();

        for num in nums {
            heap.push(num);
        }

        while let Some(num) = heap.pop() {
            result = result.min(num - min);
            if num % 2 == 1 {
                break;
            }
            min = min.min(num / 2);
            heap.push(num / 2);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1675_example_1() {
        let nums = vec![1, 2, 3, 4];

        let result = 1;

        assert_eq!(Solution::minimum_deviation(nums), result);
    }

    #[test]
    fn test_1675_example_2() {
        let nums = vec![4, 1, 5, 20, 3];

        let result = 3;

        assert_eq!(Solution::minimum_deviation(nums), result);
    }

    #[test]
    fn test_1675_example_3() {
        let nums = vec![2, 10, 8];

        let result = 3;

        assert_eq!(Solution::minimum_deviation(nums), result);
    }
}
