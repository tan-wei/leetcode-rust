/**
 * [1224] Maximum Equal Frequency
 *
 * Given an array nums of positive integers, return the longest possible length of an array prefix of nums, such that it is possible to remove exactly one element from this prefix so that every number that has appeared in it will have the same number of occurrences.
 * If after removing one element there are no remaining elements, it's still considered that every appeared number has the same number of ocurrences (0).
 *  
 * Example 1:
 *
 * Input: nums = [2,2,1,1,5,3,3,5]
 * Output: 7
 * Explanation: For the subarray [2,2,1,1,5,3,3] of length 7, if we remove nums[4] = 5, we will get [2,2,1,1,3,3], so that each number will appear exactly twice.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,2,2,2,3,3,3,4,4,4,5]
 * Output: 13
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-equal-frequency/
// discuss: https://leetcode.com/problems/maximum-equal-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-equal-frequency/solutions/2749383/rust-solution/
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut cnt_histogram = [0; 100010]; // how many we have from a given cnt
        let mut num_histogram = [0; 100010]; // number of times we saw a given number
        let mut cnt_min = 0;
        let mut cnt_max = 0;
        let mut max_len: i32 = 0;

        nums.into_iter().enumerate().for_each(|(idx, v)| {
            let v = v as usize;
            let mut c = num_histogram[v];
            if c > 0 {
                cnt_histogram[c] -= 1;
                if c == cnt_min && cnt_histogram[c] == 0 {
                    cnt_min += 1;
                }
            } else {
                cnt_min = 1;
            }
            c += 1;
            cnt_histogram[c] += 1;
            if c > cnt_max {
                cnt_max = c;
            }
            num_histogram[v] += 1;

            let n = idx + 1;

            let s_simple = { cnt_max == 1 };
            let s_single = (cnt_min == 1 && n == cnt_histogram[cnt_max] * cnt_max + 1);
            let s_over = {
                (cnt_max > 1
                    && cnt_histogram[cnt_max] == 1
                    && n == cnt_histogram[cnt_max - 1] * (cnt_max - 1) + cnt_max)
            };
            if s_simple || s_single || s_over {
                max_len = n as i32;
            }
        });

        max_len
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1224_example_1() {
        let nums = vec![2, 2, 1, 1, 5, 3, 3, 5];
        let result = 7;

        assert_eq!(Solution::max_equal_freq(nums), result);
    }

    #[test]
    fn test_1224_example_2() {
        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5];
        let result = 13;

        assert_eq!(Solution::max_equal_freq(nums), result);
    }
}
