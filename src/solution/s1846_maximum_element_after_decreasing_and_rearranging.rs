/**
 * [1846] Maximum Element After Decreasing and Rearranging
 *
 * You are given an array of positive integers arr. Perform some operations (possibly none) on arr so that it satisfies these conditions:
 *
 * 	The value of the first element in arr must be 1.
 * 	The absolute difference between any 2 adjacent elements must be less than or equal to 1. In other words, abs(arr[i] - arr[i - 1]) <= 1 for each i where 1 <= i < arr.length (0-indexed). abs(x) is the absolute value of x.
 *
 * There are 2 types of operations that you can perform any number of times:
 *
 * 	Decrease the value of any element of arr to a smaller positive integer.
 * 	Rearrange the elements of arr to be in any order.
 *
 * Return the maximum possible value of an element in arr after performing the operations to satisfy the conditions.
 *  
 * Example 1:
 *
 * Input: arr = [2,2,1,2,1]
 * Output: 2
 * Explanation:
 * We can satisfy the conditions by rearranging arr so it becomes [1,2,2,2,1].
 * The largest element in arr is 2.
 *
 * Example 2:
 *
 * Input: arr = [100,1,1000]
 * Output: 3
 * Explanation:
 * One possible way to satisfy the conditions is by doing the following:
 * 1. Rearrange arr so it becomes [1,100,1000].
 * 2. Decrease the value of the second element to 2.
 * 3. Decrease the value of the third element to 3.
 * Now arr = [1,2,3], which satisfies the conditions.
 * The largest element in arr is 3.
 *
 * Example 3:
 *
 * Input: arr = [1,2,3,4,5]
 * Output: 5
 * Explanation: The array already satisfies the conditions, and the largest element is 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/
// discuss: https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        arr.iter()
            .fold(0, |acc, x| if x > &acc { acc + 1 } else { acc })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1846_example_1() {
        let arr = vec![2, 2, 1, 2, 1];

        let result = 2;

        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(arr),
            result
        );
    }

    #[test]
    fn test_1846_example_2() {
        let arr = vec![100, 1, 1000];

        let result = 3;

        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(arr),
            result
        );
    }

    #[test]
    fn test_1846_example_3() {
        let arr = vec![1, 2, 3, 4, 5];

        let result = 5;

        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(arr),
            result
        );
    }
}
