/**
 * [1577] Number of Ways Where Square of Number Is Equal to Product of Two Numbers
 *
 * Given two arrays of integers nums1 and nums2, return the number of triplets formed (type 1 and type 2) under the following rules:
 *
 * 	Type 1: Triplet (i, j, k) if nums1[i]^2 == nums2[j] * nums2[k] where 0 <= i < nums1.length and 0 <= j < k < nums2.length.
 * 	Type 2: Triplet (i, j, k) if nums2[i]^2 == nums1[j] * nums1[k] where 0 <= i < nums2.length and 0 <= j < k < nums1.length.
 *
 *  
 * Example 1:
 *
 * Input: nums1 = [7,4], nums2 = [5,2,8,9]
 * Output: 1
 * Explanation: Type 1: (1, 1, 2), nums1[1]^2 = nums2[1] * nums2[2]. (4^2 = 2 * 8).
 *
 * Example 2:
 *
 * Input: nums1 = [1,1], nums2 = [1,1,1]
 * Output: 9
 * Explanation: All Triplets are valid, because 1^2 = 1 * 1.
 * Type 1: (0,0,1), (0,0,2), (0,1,2), (1,0,1), (1,0,2), (1,1,2).  nums1[i]^2 = nums2[j] * nums2[k].
 * Type 2: (0,0,1), (1,0,1), (2,0,1). nums2[i]^2 = nums1[j] * nums1[k].
 *
 * Example 3:
 *
 * Input: nums1 = [7,7,8,3], nums2 = [1,2,9,7]
 * Output: 2
 * Explanation: There are 2 valid triplets.
 * Type 1: (3,0,2).  nums1[3]^2 = nums2[0] * nums2[2].
 * Type 2: (3,0,1).  nums2[3]^2 = nums1[0] * nums1[1].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	1 <= nums1[i], nums2[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/
// discuss: https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;

        let (mut m1, mut m2) = (
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );

        for i in 0..nums1.len() {
            for j in i + 1..nums1.len() {
                let p = nums1[i] as i64 * nums1[j] as i64;
                *m1.entry(p).or_insert(0) += 1;
            }
        }

        for i in 0..nums2.len() {
            for j in i + 1..nums2.len() {
                let p = nums2[i] as i64 * nums2[j] as i64;
                *m2.entry(p).or_insert(0) += 1;
            }
        }

        for &num in nums1.iter() {
            if let Some(&v) = m2.get(&(num as i64 * num as i64)) {
                result += v;
            }
        }

        for &num in nums2.iter() {
            if let Some(&v) = m1.get(&(num as i64 * num as i64)) {
                result += v;
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
    fn test_1577_example_1() {
        let nums1 = vec![7, 4];
        let nums2 = vec![5, 2, 8, 9];

        let result = 1;

        assert_eq!(Solution::num_triplets(nums1, nums2), result);
    }

    #[test]
    fn test_1577_example_2() {
        let nums1 = vec![1, 1];
        let nums2 = vec![1, 1, 1];

        let result = 9;

        assert_eq!(Solution::num_triplets(nums1, nums2), result);
    }

    #[test]
    fn test_1577_example_3() {
        let nums1 = vec![7, 7, 8, 3];
        let nums2 = vec![1, 2, 9, 7];

        let result = 2;

        assert_eq!(Solution::num_triplets(nums1, nums2), result);
    }
}
