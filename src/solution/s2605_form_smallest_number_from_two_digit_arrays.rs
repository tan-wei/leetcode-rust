/**
 * [2605] Form Smallest Number From Two Digit Arrays
 *
 * Given two arrays of unique digits nums1 and nums2, return the smallest number that contains at least one digit from each array.
 *  
 * Example 1:
 *
 * Input: nums1 = [4,1,3], nums2 = [5,7]
 * Output: 15
 * Explanation: The number 15 contains the digit 1 from nums1 and the digit 5 from nums2. It can be proven that 15 is the smallest number we can have.
 *
 * Example 2:
 *
 * Input: nums1 = [3,5,2,6], nums2 = [3,1,7]
 * Output: 3
 * Explanation: The number 3 contains the digit 3 which exists in both arrays.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 9
 * 	1 <= nums1[i], nums2[i] <= 9
 * 	All digits in each array are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/form-smallest-number-from-two-digit-arrays/
// discuss: https://leetcode.com/problems/form-smallest-number-from-two-digit-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let result = match nums1.iter().filter(|x| nums2.contains(*x)).min() {
            Some(&x) => x,
            _ => {
                let (a, b) = (
                    nums1.into_iter().min().unwrap(),
                    nums2.into_iter().min().unwrap(),
                );
                10 * a.min(b) + a.max(b)
            }
        };

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2605_example_1() {
        let nums1 = vec![4, 1, 3];
        let nums2 = vec![5, 7];

        let result = 15;

        assert_eq!(Solution::min_number(nums1, nums2), result);
    }

    #[test]
    fn test_2605_example_2() {
        let nums1 = vec![3, 5, 2, 6];
        let nums2 = vec![3, 1, 7];

        let result = 3;

        assert_eq!(Solution::min_number(nums1, nums2), result);
    }
}
