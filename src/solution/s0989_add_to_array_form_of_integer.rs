/**
 * [0989] Add to Array-Form of Integer
 *
 * The array-form of an integer num is an array representing its digits in left to right order.
 *
 * 	For example, for num = 1321, the array form is [1,3,2,1].
 *
 * Given num, the array-form of an integer, and an integer k, return the array-form of the integer num + k.
 *  
 * Example 1:
 *
 * Input: num = [1,2,0,0], k = 34
 * Output: [1,2,3,4]
 * Explanation: 1200 + 34 = 1234
 *
 * Example 2:
 *
 * Input: num = [2,7,4], k = 181
 * Output: [4,5,5]
 * Explanation: 274 + 181 = 455
 *
 * Example 3:
 *
 * Input: num = [2,1,5], k = 806
 * Output: [1,0,2,1]
 * Explanation: 215 + 806 = 1021
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 10^4
 * 	0 <= num[i] <= 9
 * 	num does not contain any leading zeros except for the zero itself.
 * 	1 <= k <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-to-array-form-of-integer/
// discuss: https://leetcode.com/problems/add-to-array-form-of-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num: Vec<i32> = num.into_iter().rev().collect();

        let (mut carry, mut index) = (k, 0);

        while carry > 0 {
            if index >= num.len() {
                num.push(0);
            }

            let tmp = num[index] + carry;

            num[index] = tmp % 10;
            carry = tmp / 10;
            index += 1
        }

        num.into_iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0989_example_1() {
        let num = vec![1, 2, 0, 0];
        let k = 34;
        let result = vec![1, 2, 3, 4];

        assert_eq!(Solution::add_to_array_form(num, k), result);
    }

    #[test]
    fn test_0989_example_2() {
        let num = vec![2, 7, 4];
        let k = 181;
        let result = vec![4, 5, 5];

        assert_eq!(Solution::add_to_array_form(num, k), result);
    }

    #[test]
    fn test_0989_example_3() {
        let num = vec![2, 1, 5];
        let k = 806;
        let result = vec![1, 0, 2, 1];

        assert_eq!(Solution::add_to_array_form(num, k), result);
    }
}
