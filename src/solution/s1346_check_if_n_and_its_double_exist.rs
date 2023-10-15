/**
 * [1346] Check If N and Its Double Exist
 *
 * Given an array arr of integers, check if there exist two indices i and j such that :
 *
 * 	i != j
 * 	0 <= i, j < arr.length
 * 	arr[i] == 2 * arr[j]
 *
 *  
 * Example 1:
 *
 * Input: arr = [10,2,5,3]
 * Output: true
 * Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
 *
 * Example 2:
 *
 * Input: arr = [3,1,7,11]
 * Output: false
 * Explanation: There is no i and j that satisfy the conditions.
 *
 *  
 * Constraints:
 *
 * 	2 <= arr.length <= 500
 * 	-10^3 <= arr[i] <= 10^3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-n-and-its-double-exist/
// discuss: https://leetcode.com/problems/check-if-n-and-its-double-exist/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
        let mut set = std::collections::HashSet::new();

        for i in 0..arr.len() {
            if set.contains(&(arr[i] * 2)) {
                return true;
            }
            set.insert(arr[i]);
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1346_example_1() {
        let arr = vec![10, 2, 5, 3];

        let result = true;

        assert_eq!(Solution::check_if_exist(arr), result);
    }

    #[test]
    fn test_1346_example_2() {
        let arr = vec![3, 1, 7, 11];

        let result = false;

        assert_eq!(Solution::check_if_exist(arr), result);
    }
}
