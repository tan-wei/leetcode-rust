/**
 * [1052] Grumpy Bookstore Owner
 *
 * There is a bookstore owner that has a store open for n minutes. Every minute, some number of customers enter the store. You are given an integer array customers of length n where customers[i] is the number of the customer that enters the store at the start of the i^th minute and all those customers leave after the end of that minute.
 * On some minutes, the bookstore owner is grumpy. You are given a binary array grumpy where grumpy[i] is 1 if the bookstore owner is grumpy during the i^th minute, and is 0 otherwise.
 * When the bookstore owner is grumpy, the customers of that minute are not satisfied, otherwise, they are satisfied.
 * The bookstore owner knows a secret technique to keep themselves not grumpy for minutes consecutive minutes, but can only use it once.
 * Return the maximum number of customers that can be satisfied throughout the day.
 *  
 * Example 1:
 *
 * Input: customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3
 * Output: 16
 * Explanation: The bookstore owner keeps themselves not grumpy for the last 3 minutes.
 * The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.
 *
 * Example 2:
 *
 * Input: customers = [1], grumpy = [0], minutes = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == customers.length == grumpy.length
 * 	1 <= minutes <= n <= 2 * 10^4
 * 	0 <= customers[i] <= 1000
 * 	grumpy[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/grumpy-bookstore-owner/
// discuss: https://leetcode.com/problems/grumpy-bookstore-owner/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        let mut maxsum = 0;
        for i in 0..customers.len() {
            let idx = (i as i32) - minutes;
            sum += customers[i] * grumpy[i];
            if idx >= 0 {
                sum -= customers[idx as usize] * grumpy[idx as usize];
            }
            maxsum = std::cmp::max(maxsum, sum);
            result += customers[i] * -(grumpy[i] - 1);
        }
        result + maxsum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1052_example_1() {
        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes = 3;
        let result = 16;

        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), result);
    }

    #[test]
    fn test_1052_example_2() {
        let customers = vec![1];
        let grumpy = vec![0];
        let minutes = 1;
        let result = 1;

        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), result);
    }
}
