/**
 * [1442] Count Triplets That Can Form Two Arrays of Equal XOR
 *
 * Given an array of integers arr.
 * We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).
 * Let's define a and b as follows:
 *
 * 	a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
 * 	b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
 *
 * Note that ^ denotes the bitwise-xor operation.
 * Return the number of triplets (i, j and k) Where a == b.
 *  
 * Example 1:
 *
 * Input: arr = [2,3,1,6,7]
 * Output: 4
 * Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
 *
 * Example 2:
 *
 * Input: arr = [1,1,1,1,1]
 * Output: 10
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 300
 * 	1 <= arr[i] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
// discuss: https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..(arr.len() - 1) {
            let mut current = arr[i];
            for k in (i + 1)..arr.len() {
                current ^= arr[k];
                if current == 0 {
                    result += k - i;
                }
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1442_example_1() {
        let arr = vec![2, 3, 1, 6, 7];

        let result = 4;

        assert_eq!(Solution::count_triplets(arr), result);
    }

    #[test]
    fn test_1442_example_2() {
        let arr = vec![1, 1, 1, 1, 1];

        let result = 10;

        assert_eq!(Solution::count_triplets(arr), result);
    }
}
