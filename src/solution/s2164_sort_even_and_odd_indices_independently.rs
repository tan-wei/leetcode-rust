/**
 * [2164] Sort Even and Odd Indices Independently
 *
 * You are given a 0-indexed integer array nums. Rearrange the values of nums according to the following rules:
 * <ol>
 * 	Sort the values at odd indices of nums in non-increasing order.
 *
 * 		For example, if nums = [4,<u>1</u>,2,<u>3</u>] before this step, it becomes [4,<u>3</u>,2,<u>1</u>] after. The values at odd indices 1 and 3 are sorted in non-increasing order.
 *
 *
 * 	Sort the values at even indices of nums in non-decreasing order.
 *
 * 		For example, if nums = [<u>4</u>,1,<u>2</u>,3] before this step, it becomes [<u>2</u>,1,<u>4</u>,3] after. The values at even indices 0 and 2 are sorted in non-decreasing order.
 *
 *
 * </ol>
 * Return the array formed after rearranging the values of nums.
 *  
 * Example 1:
 *
 * Input: nums = [4,1,2,3]
 * Output: [2,3,4,1]
 * Explanation:
 * First, we sort the values present at odd indices (1 and 3) in non-increasing order.
 * So, nums changes from [4,<u>1</u>,2,<u>3</u>] to [4,<u>3</u>,2,<u>1</u>].
 * Next, we sort the values present at even indices (0 and 2) in non-decreasing order.
 * So, nums changes from [<u>4</u>,1,<u>2</u>,3] to [<u>2</u>,3,<u>4</u>,1].
 * Thus, the array formed after rearranging the values is [2,3,4,1].
 *
 * Example 2:
 *
 * Input: nums = [2,1]
 * Output: [2,1]
 * Explanation:
 * Since there is exactly one odd index and one even index, no rearrangement of values takes place.
 * The resultant array formed is [2,1], which is the same as the initial array.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 100
 * 	1 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-even-and-odd-indices-independently/
// discuss: https://leetcode.com/problems/sort-even-and-odd-indices-independently/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        let mut even: Vec<i32> = nums.iter().step_by(2).map(|&x| x).collect();
        even.sort_by(|a, b| a.cmp(&b));
        let mut e = even.iter();

        let mut odd: Vec<i32> = nums.iter().skip(1).step_by(2).map(|&x| x).collect();
        odd.sort_by(|a, b| b.cmp(&a));
        let mut o = odd.iter();

        for i in 0..nums.len() {
            match i % 2 {
                0 => result.push(*e.next().unwrap()),
                _ => result.push(*o.next().unwrap()),
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2164_example_1() {
        let nums = vec![4, 1, 2, 3];

        let result = vec![2, 3, 4, 1];

        assert_eq!(Solution::sort_even_odd(nums), result);
    }

    #[test]
    fn test_2164_example_2() {
        let nums = vec![2, 1];

        let result = vec![2, 1];

        assert_eq!(Solution::sort_even_odd(nums), result);
    }
}
