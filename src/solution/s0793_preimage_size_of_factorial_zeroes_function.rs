/**
 * [0793] Preimage Size of Factorial Zeroes Function
 *
 * Let f(x) be the number of zeroes at the end of x!. Recall that x! = 1 * 2 * 3 * ... * x and by convention, 0! = 1.
 *
 * 	For example, f(3) = 0 because 3! = 6 has no zeroes at the end, while f(11) = 2 because 11! = 39916800 has two zeroes at the end.
 *
 * Given an integer k, return the number of non-negative integers x have the property that f(x) = k.
 *  
 * Example 1:
 *
 * Input: k = 0
 * Output: 5
 * Explanation: 0!, 1!, 2!, 3!, and 4! end with k = 0 zeroes.
 *
 * Example 2:
 *
 * Input: k = 5
 * Output: 0
 * Explanation: There is no x such that x! ends in k = 5 zeroes.
 *
 * Example 3:
 *
 * Input: k = 3
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	0 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function/
// discuss: https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let mut nums = vec![0; 13];
        let mut k = k;
        nums[0] = 1;

        for i in 1..13 {
            nums[i] = 5 * nums[i - 1] + 1;
        }

        for i in (0..13).rev() {
            let num = nums[i];
            if k / num == 5 {
                return 0;
            }
            k %= nums[i];
        }
        5
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0793_example_1() {
        let k = 0;
        let result = 5;

        assert_eq!(Solution::preimage_size_fzf(k), result);
    }

    #[test]
    fn test_0793_example_2() {
        let k = 5;
        let result = 0;

        assert_eq!(Solution::preimage_size_fzf(k), result);
    }

    #[test]
    fn test_0793_example_3() {
        let k = 3;
        let result = 5;

        assert_eq!(Solution::preimage_size_fzf(k), result);
    }
}
