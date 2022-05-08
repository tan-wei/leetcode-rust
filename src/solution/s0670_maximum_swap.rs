/**
 * [0670] Maximum Swap
 *
 * You are given an integer num. You can swap two digits at most once to get the maximum valued number.
 * Return the maximum valued number you can get.
 *  
 * Example 1:
 *
 * Input: num = 2736
 * Output: 7236
 * Explanation: Swap the number 2 and the number 7.
 *
 * Example 2:
 *
 * Input: num = 9973
 * Output: 9973
 * Explanation: No swap.
 *
 *  
 * Constraints:
 *
 * 	0 <= num <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-swap/
// discuss: https://leetcode.com/problems/maximum-swap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut nums = num.to_string().chars().collect::<Vec<char>>();
        let mut last = vec![0; 10];

        for i in 0..nums.len() {
            last[(nums[i] as u8 - b'0') as usize] = i;
        }

        for i in 0..nums.len() {
            let digit = (nums[i] as u8 - b'0') as usize;
            for j in (digit + 1..10).rev() {
                if last[j] > i {
                    nums.swap(i, last[j]);
                    return nums.into_iter().collect::<String>().parse::<i32>().unwrap();
                }
            }
        }

        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0670_example_1() {
        let num = 2736;
        let result = 7236;

        assert_eq!(Solution::maximum_swap(num), result);
    }

    #[test]
    fn test_0670_example_2() {
        let num = 9973;
        let result = 9973;

        assert_eq!(Solution::maximum_swap(num), result);
    }
}
