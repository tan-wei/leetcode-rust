/**
 * [1287] Element Appearing More Than 25% In Sorted Array
 *
 * Given an integer array sorted in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time, return that integer.
 *
 * Example 1:
 *
 * Input: arr = [1,2,2,6,6,6,6,7,10]
 * Output: 6
 *
 * Example 2:
 *
 * Input: arr = [1,1]
 * Output: 1
 *
 *
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	0 <= arr[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
// discuss: https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut m: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in arr.iter() {
            *m.entry(*i).or_insert(0) += 1;
        }
        let (mut k, mut max) = (0, 0);
        for (key, val) in m {
            if max < val {
                max = val;
                k = key;
            }
        }
        k
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1287_example_1() {
        let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
        let result = 6;

        assert_eq!(Solution::find_special_integer(arr), result);
    }

    #[test]
    fn test_1287_example_2() {
        let arr = vec![1, 1];
        let result = 1;

        assert_eq!(Solution::find_special_integer(arr), result);
    }
}
