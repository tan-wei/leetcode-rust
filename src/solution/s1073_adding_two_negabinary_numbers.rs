/**
 * [1073] Adding Two Negabinary Numbers
 *
 * Given two numbers arr1 and arr2 in base -2, return the result of adding them together.
 * Each number is given in array format:  as an array of 0s and 1s, from most significant bit to least significant bit.  For example, arr = [1,1,0,1] represents the number (-2)^3 + (-2)^2 + (-2)^0 = -3.  A number arr in array, format is also guaranteed to have no leading zeros: either arr == [0] or arr[0] == 1.
 * Return the result of adding arr1 and arr2 in the same format: as an array of 0s and 1s with no leading zeros.
 *  
 * Example 1:
 *
 * Input: arr1 = [1,1,1,1,1], arr2 = [1,0,1]
 * Output: [1,0,0,0,0]
 * Explanation: arr1 represents 11, arr2 represents 5, the output represents 16.
 *
 * Example 2:
 *
 * Input: arr1 = [0], arr2 = [0]
 * Output: [0]
 *
 * Example 3:
 *
 * Input: arr1 = [0], arr2 = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= arr1.length, arr2.length <= 1000
 * 	arr1[i] and arr2[i] are 0 or 1
 * 	arr1 and arr2 have no leading zeros
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/adding-two-negabinary-numbers/
// discuss: https://leetcode.com/problems/adding-two-negabinary-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for d in arr2.iter().rev() {
            result.push(*d);
        }

        for (i, d) in arr1.iter().rev().enumerate() {
            if i >= result.len() {
                result.push(*d);
            } else {
                result[i] += d;
            }
        }

        let mut i = 0;
        while i < result.len() {
            if result[i] < 0 || result[i] > 1 {
                if i + 1 == result.len() {
                    result.push(0)
                }
            }

            if result[i] < 0 {
                result[i] = 1;
                result[i + 1] += 1;
            } else if result[i] >= 2 {
                result[i] -= 2;
                result[i + 1] -= 1;
            }

            i += 1;
        }

        result = result.into_iter().rev().collect::<Vec<i32>>();

        while result[0] == 0 && result.len() != 1 {
            result.remove(0);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1073_example_1() {
        let arr1 = vec![1, 1, 1, 1, 1];
        let arr2 = vec![1, 0, 1];
        let result = vec![1, 0, 0, 0, 0];

        assert_eq!(Solution::add_negabinary(arr1, arr2), result);
    }

    #[test]
    fn test_1073_example_2() {
        let arr1 = vec![0];
        let arr2 = vec![0];
        let result = vec![0];

        assert_eq!(Solution::add_negabinary(arr1, arr2), result);
    }

    #[test]
    fn test_1073_example_3() {
        let arr1 = vec![0];
        let arr2 = vec![1];
        let result = vec![1];

        assert_eq!(Solution::add_negabinary(arr1, arr2), result);
    }
}
