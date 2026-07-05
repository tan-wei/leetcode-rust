/**
 * [2530] Maximal Score After Applying K Operations
 *
 * You are given a 0-indexed integer array nums and an integer k. You have a starting score of 0.
 * In one operation:
 * <ol>
 * 	choose an index i such that 0 <= i < nums.length,
 * 	increase your score by nums[i], and
 * 	replace nums[i] with ceil(nums[i] / 3).
 * </ol>
 * Return the maximum possible score you can attain after applying exactly k operations.
 * The ceiling function ceil(val) is the least integer greater than or equal to val.
 *  
 * Example 1:
 *
 * Input: nums = [10,10,10,10,10], k = 5
 * Output: 50
 * Explanation: Apply the operation to each array element exactly once. The final score is 10 + 10 + 10 + 10 + 10 = 50.
 *
 * Example 2:
 *
 * Input: nums = [1,10,3,3,3], k = 3
 * Output: 17
 * Explanation: You can do the following operations:
 * Operation 1: Select i = 1, so nums becomes [1,<u>4</u>,3,3,3]. Your score increases by 10.
 * Operation 2: Select i = 1, so nums becomes [1,<u>2</u>,3,3,3]. Your score increases by 4.
 * Operation 3: Select i = 2, so nums becomes [1,2,<u>1</u>,3,3]. Your score increases by 3.
 * The final score is 10 + 4 + 3 = 17.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length, k <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-score-after-applying-k-operations/
// discuss: https://leetcode.com/problems/maximal-score-after-applying-k-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximal-score-after-applying-k-operations/solutions/5911063/rust-20ms-458mb-one-liner-solution-by-ro-2ovc/
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        (0..k)
            .fold(
                (
                    0,
                    nums.into_iter()
                        .collect::<std::collections::BinaryHeap<i32>>(),
                ),
                |(ans, mut v), _| match v.peek() == Some(&1) {
                    true => (ans + 1, v),
                    false => {
                        let n = v.pop().unwrap_or(0) as i64;
                        v.push((n as f64 / 3.0).ceil() as i32);
                        (ans + n, v)
                    }
                },
            )
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2530_example_1() {
        let nums = vec![10, 10, 10, 10, 10];
        let k = 5;

        let result = 50;

        assert_eq!(Solution::max_kelements(nums, k), result);
    }

    #[test]
    fn test_2530_example_2() {
        let nums = vec![1, 10, 3, 3, 3];
        let k = 3;

        let result = 17;

        assert_eq!(Solution::max_kelements(nums, k), result);
    }
}
