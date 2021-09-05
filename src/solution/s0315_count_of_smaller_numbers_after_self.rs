/**
 * [315] Count of Smaller Numbers After Self
 *
 * You are given an integer array nums and you have to return a new counts array. The counts array has the property where counts[i] is the number of smaller elements to the right of nums[i].
 *  
 * Example 1:
 *
 * Input: nums = [5,2,6,1]
 * Output: [2,1,1,0]
 * Explanation:
 * To the right of 5 there are 2 smaller elements (2 and 1).
 * To the right of 2 there is only 1 smaller element (1).
 * To the right of 6 there is 1 smaller element (1).
 * To the right of 1 there is 0 smaller element.
 *
 * Example 2:
 *
 * Input: nums = [-1]
 * Output: [0]
 *
 * Example 3:
 *
 * Input: nums = [-1,-1]
 * Output: [0,0]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-smaller-numbers-after-self/
// discuss: https://leetcode.com/problems/count-of-smaller-numbers-after-self/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Creit: https://leetcode.com/problems/count-of-smaller-numbers-after-self/discuss/889481/24-ms-Rust-simple-solution
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        use std::convert::{TryFrom, TryInto};
        let mut numbers_left = {
            let unique_numbers: HashSet<_> = nums.iter().copied().collect();
            let mut num_counter: Vec<(i32, u32)> = unique_numbers.iter().map(|&x| (x, 0)).collect();
            num_counter.sort_unstable();
            num_counter
        };
        let mut res = vec![0i32; nums.len()];
        for (i, &v) in nums.iter().enumerate().rev() {
            let position_number = numbers_left.binary_search_by_key(&v, |&(k, _)| k).unwrap();
            let res_count = if position_number <= nums.len() - i {
                numbers_left[..position_number]
                    .iter()
                    .map(|(_, c)| c)
                    .sum::<u32>()
            } else {
                nums[i + 1..]
                    .iter()
                    .filter(|&&x| x < v)
                    .count()
                    .try_into()
                    .unwrap()
            };
            numbers_left[position_number].1 += 1;
            res[i] = res_count.try_into().unwrap();
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0315_exmaple_1() {
        let nums = vec![5, 2, 6, 1];
        let result = vec![2, 1, 1, 0];

        assert_eq!(Solution::count_smaller(nums), result);
    }

    #[test]
    fn test_0315_exmaple_2() {
        let nums = vec![-1];
        let result = vec![0];

        assert_eq!(Solution::count_smaller(nums), result);
    }

    #[test]
    fn test_0315_exmaple_3() {
        let nums = vec![-1, -1];
        let result = vec![0, 0];

        assert_eq!(Solution::count_smaller(nums), result);
    }
}
