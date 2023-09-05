/**
 * [1299] Replace Elements with Greatest Element on Right Side
 *
 * Given an array arr, replace every element in that array with the greatest element among the elements to its right, and replace the last element with -1.
 * After doing so, return the array.
 *
 * Example 1:
 *
 * Input: arr = [17,18,5,4,6,1]
 * Output: [18,6,6,6,1,-1]
 * Explanation:
 * - index 0 --> the greatest element to the right of index 0 is index 1 (18).
 * - index 1 --> the greatest element to the right of index 1 is index 4 (6).
 * - index 2 --> the greatest element to the right of index 2 is index 4 (6).
 * - index 3 --> the greatest element to the right of index 3 is index 4 (6).
 * - index 4 --> the greatest element to the right of index 4 is index 5 (1).
 * - index 5 --> there are no elements to the right of index 5, so we put -1.
 *
 * Example 2:
 *
 * Input: arr = [400]
 * Output: [-1]
 * Explanation: There are no elements to the right of index 0.
 *
 *
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	1 <= arr[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
// discuss: https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut mx = -1;
        let mut result = vec![-1; arr.len()];

        for i in (0..arr.len()).rev() {
            result[i] = mx;
            mx = std::cmp::max(mx, arr[i]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1299_example_1() {
        let arr = vec![17, 18, 5, 4, 6, 1];
        let result = vec![18, 6, 6, 6, 1, -1];

        assert_eq!(Solution::replace_elements(arr), result);
    }

    #[test]
    fn test_1299_example_2() {
        let arr = vec![400];
        let result = vec![-1];

        assert_eq!(Solution::replace_elements(arr), result);
    }
}
