/**
 * [1089] Duplicate Zeros
 *
 * Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.
 * Note that elements beyond the length of the original array are not written. Do the above modifications to the input array in place and do not return anything.
 *  
 * Example 1:
 *
 * Input: arr = [1,0,2,3,0,4,5,0]
 * Output: [1,0,0,2,3,0,0,4]
 * Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
 *
 * Example 2:
 *
 * Input: arr = [1,2,3]
 * Output: [1,2,3]
 * Explanation: After calling your function, the input array is modified to: [1,2,3]
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	0 <= arr[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/duplicate-zeros/
// discuss: https://leetcode.com/problems/duplicate-zeros/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut index = 0..arr.len();
        while let Some(i) = index.next() {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                arr.remove(arr.len() - 1);
                index.next();
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1089_example_1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        let result = vec![1, 0, 0, 2, 3, 0, 0, 4];

        Solution::duplicate_zeros(&mut arr);

        assert_eq!(arr, result);
    }

    #[test]
    fn test_1089_example_2() {
        let mut arr = vec![1, 2, 3];
        let result = vec![1, 2, 3];

        Solution::duplicate_zeros(&mut arr);

        assert_eq!(arr, result);
    }
}
