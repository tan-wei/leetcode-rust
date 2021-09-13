/**
 * [327] Count of Range Sum
 *
 * Given an integer array nums and two integers lower and upper, return the number of range sums that lie in [lower, upper] inclusive.
 * Range sum S(i, j) is defined as the sum of the elements in nums between indices i and j inclusive, where i <= j.
 *  
 * Example 1:
 *
 * Input: nums = [-2,5,-1], lower = -2, upper = 2
 * Output: 3
 * Explanation: The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
 *
 * Example 2:
 *
 * Input: nums = [0], lower = 0, upper = 0
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	-10^5 <= lower <= upper <= 10^5
 * 	The answer is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-range-sum/
// discuss: https://leetcode.com/problems/count-of-range-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-of-range-sum/discuss/1378661/Rust-Divide-and-Conquer-using-prefix-sum
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        Self::count_sub_range_sum(
            std::rc::Rc::new(std::cell::RefCell::new(
                Some(0 as i64)
                    .iter()
                    .chain(nums.iter().map(|x| *x as i64).collect::<Vec<i64>>().iter())
                    .scan(0, |state, x| {
                        *state += x;
                        Some(*state as i64)
                    })
                    .collect::<Vec<i64>>(),
            )),
            0,
            nums.len(),
            lower as i64,
            upper as i64,
        )
    }

    fn count_sub_range_sum(
        prefix: std::rc::Rc<std::cell::RefCell<Vec<i64>>>,
        start: usize,
        end: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if start >= end {
            return 0;
        }

        // Pre-compute the two halves, thus assuming both sorted
        let mid: usize = (start + end) / 2;
        let mut count: i32 =
            Self::count_sub_range_sum(std::rc::Rc::clone(&prefix), start, mid, lower, upper)
                + Self::count_sub_range_sum(
                    std::rc::Rc::clone(&prefix),
                    mid + 1,
                    end,
                    lower,
                    upper,
                );

        // loop over every pos in left and use two pointers for lower & upper
        let (mut i, mut j) = (mid + 1, mid + 1);
        for pos in start..=mid {
            // compute the range of acceptable values
            while i <= end && (*prefix.borrow())[i] - (*prefix.borrow())[pos] < lower {
                i += 1;
            }
            while j <= end && (*prefix.borrow())[j] - (*prefix.borrow())[pos] <= upper {
                j += 1;
            }
            count += (j - i) as i32;
        }

        // partially sort the vector
        let mut replacement = (*prefix.borrow())[start..=end].to_vec();
        replacement.sort();
        (*prefix.borrow_mut())[start..=end].copy_from_slice(&replacement[..]);

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0327_example_1() {
        let nums = vec![-2, 5, -1];
        let lower = -2;
        let upper = 2;

        let result = 3;

        assert_eq!(Solution::count_range_sum(nums, lower, upper), result);
    }

    #[test]
    fn test_0327_example_2() {
        let nums = vec![0];
        let lower = 0;
        let upper = 0;

        let result = 1;

        assert_eq!(Solution::count_range_sum(nums, lower, upper), result);
    }
}
