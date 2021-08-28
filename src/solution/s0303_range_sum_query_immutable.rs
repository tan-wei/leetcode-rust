/**
 * [303] Range Sum Query - Immutable
 *
 * Given an integer array nums, handle multiple queries of the following type:
 * <ol>
 * 	Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
 * </ol>
 * Implement the NumArray class:
 *
 * 	NumArray(int[] nums) Initializes the object with the integer array nums.
 * 	int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
 *
 *  
 * Example 1:
 *
 * Input
 * ["NumArray", "sumRange", "sumRange", "sumRange"]
 * [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
 * Output
 * [null, 1, -1, -3]
 * Explanation
 * NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
 * numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
 * numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
 * numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
 *
 *
 *
 *
 *  
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^5 <= nums[i] <= 10^5
 * 	0 <= left <= right < nums.length
 * 	At most 10^4 calls will be made to sumRange.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-immutable/
// discuss: https://leetcode.com/problems/range-sum-query-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray(Vec<i32>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray(
            nums.into_iter()
                .scan(0, |mut state, x| {
                    *state += x;
                    Some(*state)
                })
                .collect(),
        )
    }
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let i = left as usize;
        let j = right as usize;

        if i == 0 {
            self.0[j]
        } else {
            self.0[j] - self.0[i - 1]
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0303_example_1() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
