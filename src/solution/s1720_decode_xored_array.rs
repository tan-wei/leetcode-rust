/**
 * [1720] Decode XORed Array
 *
 * There is a hidden integer array arr that consists of n non-negative integers.
 * It was encoded into another integer array encoded of length n - 1, such that encoded[i] = arr[i] XOR arr[i + 1]. For example, if arr = [1,0,2,1], then encoded = [1,2,3].
 * You are given the encoded array. You are also given an integer first, that is the first element of arr, i.e. arr[0].
 * Return the original array arr. It can be proved that the answer exists and is unique.
 *  
 * Example 1:
 *
 * Input: encoded = [1,2,3], first = 1
 * Output: [1,0,2,1]
 * Explanation: If arr = [1,0,2,1], then first = 1 and encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]
 *
 * Example 2:
 *
 * Input: encoded = [6,2,7,3], first = 4
 * Output: [4,2,0,7,4]
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^4
 * 	encoded.length == n - 1
 * 	0 <= encoded[i] <= 10^5
 * 	0 <= first <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-xored-array/
// discuss: https://leetcode.com/problems/decode-xored-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        encoded.into_iter().fold(vec![first], |mut acc, i| {
            acc.push(acc[acc.len() - 1] ^ i);
            acc
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1720_example_1() {
        let encoded = vec![1, 2, 3];
        let first = 1;

        let result = vec![1, 0, 2, 1];

        assert_eq!(Solution::decode(encoded, first), result);
    }

    #[test]
    fn test_1720_example_2() {
        let encoded = vec![6, 2, 7, 3];
        let first = 4;

        let result = vec![4, 2, 0, 7, 4];

        assert_eq!(Solution::decode(encoded, first), result);
    }
}
