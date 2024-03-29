/**
 * [1539] Kth Missing Positive Number
 *
 * Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
 * Return the k^th positive integer that is missing from this array.
 *  
 * Example 1:
 *
 * Input: arr = [2,3,4,7,11], k = 5
 * Output: 9
 * Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5^th missing positive integer is 9.
 *
 * Example 2:
 *
 * Input: arr = [1,2,3,4], k = 2
 * Output: 6
 * Explanation: The missing positive integers are [5,6,7,...]. The 2^nd missing positive integer is 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 1000
 * 	1 <= arr[i] <= 1000
 * 	1 <= k <= 1000
 * 	arr[i] < arr[j] for 1 <= i < j <= arr.length
 *
 *  
 * Follow up:
 * Could you solve this problem in less than O(n) complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-missing-positive-number/
// discuss: https://leetcode.com/problems/kth-missing-positive-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        arr.iter()
            .enumerate()
            .find(|(i, &val)| val - *i as i32 > k)
            .unwrap_or((arr.len(), &0))
            .0 as i32
            + k
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1539_example_1() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;

        let result = 9;

        assert_eq!(Solution::find_kth_positive(arr, k), result);
    }

    #[test]
    fn test_1539_example_2() {
        let arr = vec![1, 2, 3, 4];
        let k = 2;

        let result = 6;

        assert_eq!(Solution::find_kth_positive(arr, k), result);
    }
}
