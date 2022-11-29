/**
 * [0898] Bitwise ORs of Subarrays
 *
 * We have an array arr of non-negative integers.
 * For every (contiguous) subarray sub = [arr[i], arr[i + 1], ..., arr[j]] (with i <= j), we take the bitwise OR of all the elements in sub, obtaining a result arr[i] | arr[i + 1] | ... | arr[j].
 * Return the number of possible results. Results that occur more than once are only counted once in the final answer
 *  
 * Example 1:
 *
 * Input: arr = [0]
 * Output: 1
 * Explanation: There is only one possible result: 0.
 *
 * Example 2:
 *
 * Input: arr = [1,1,2]
 * Output: 3
 * Explanation: The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
 * These yield the results 1, 1, 2, 1, 3, 3.
 * There are 3 unique values, so the answer is 3.
 *
 * Example 3:
 *
 * Input: arr = [1,2,4]
 * Output: 6
 * Explanation: The possible results are 1, 2, 3, 4, 6, and 7.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-ors-of-subarrays/
// discuss: https://leetcode.com/problems/bitwise-ors-of-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut hs = std::collections::HashSet::new();
        let n = arr.len();
        hs.insert(arr[0]);
        for i in 1..n {
            let mut pre_or = 0;
            let mut curr_or = arr[i];
            let mut j = (i - 1) as i32;
            hs.insert(curr_or);
            while 0 <= j && curr_or != pre_or {
                let uj = j as usize;
                curr_or |= arr[uj];
                pre_or |= arr[uj];
                hs.insert(curr_or);
                j -= 1;
            }
        }
        hs.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0898_example_1() {
        let arr = vec![0];
        let result = 1;

        assert_eq!(Solution::subarray_bitwise_o_rs(arr), result);
    }

    #[test]
    fn test_0898_example_2() {
        let arr = vec![1, 1, 2];
        let result = 3;

        assert_eq!(Solution::subarray_bitwise_o_rs(arr), result);
    }

    #[test]
    fn test_0898_example_3() {
        let arr = vec![1, 2, 4];
        let result = 6;

        assert_eq!(Solution::subarray_bitwise_o_rs(arr), result);
    }
}
