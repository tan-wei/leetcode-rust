/**
 * [1471] The k Strongest Values in an Array
 *
 * Given an array of integers arr and an integer k.
 * A value arr[i] is said to be stronger than a value arr[j] if |arr[i] - m| > |arr[j] - m| where m is the median of the array.<br />
 * If |arr[i] - m| == |arr[j] - m|, then arr[i] is said to be stronger than arr[j] if arr[i] > arr[j].
 * Return a list of the strongest k values in the array. return the answer in any arbitrary order.
 * Median is the middle value in an ordered integer list. More formally, if the length of the list is n, the median is the element in position ((n - 1) / 2) in the sorted list (0-indexed).
 *
 * 	For arr = [6, -3, 7, 2, 11], n = 5 and the median is obtained by sorting the array arr = [-3, 2, 6, 7, 11] and the median is arr[m] where m = ((5 - 1) / 2) = 2. The median is 6.
 * 	For arr = [-7, 22, 17,&thinsp;3], n = 4 and the median is obtained by sorting the array arr = [-7, 3, 17, 22] and the median is arr[m] where m = ((4 - 1) / 2) = 1. The median is 3.
 *
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,4,5], k = 2
 * Output: [5,1]
 * Explanation: Median is 3, the elements of the array sorted by the strongest are [5,1,4,2,3]. The strongest 2 elements are [5, 1]. [1, 5] is also accepted answer.
 * Please note that although |5 - 3| == |1 - 3| but 5 is stronger than 1 because 5 > 1.
 *
 * Example 2:
 *
 * Input: arr = [1,1,3,5,5], k = 2
 * Output: [5,5]
 * Explanation: Median is 3, the elements of the array sorted by the strongest are [5,5,1,1,3]. The strongest 2 elements are [5, 5].
 *
 * Example 3:
 *
 * Input: arr = [6,7,11,7,6,8], k = 5
 * Output: [11,8,6,6,7]
 * Explanation: Median is 7, the elements of the array sorted by the strongest are [11,8,6,6,7,7].
 * Any permutation of [11,8,6,6,7] is accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	-10^5 <= arr[i] <= 10^5
 * 	1 <= k <= arr.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-k-strongest-values-in-an-array/
// discuss: https://leetcode.com/problems/the-k-strongest-values-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();

        let n = arr.len();
        let median = arr[(n - 1) / 2];

        arr.sort_by(|a, b| {
            let v = (a - median).abs().cmp(&(b - median).abs());
            if v == std::cmp::Ordering::Equal {
                a.cmp(&b)
            } else {
                v
            }
        });

        let mut result = vec![];
        for i in 0..k {
            result.push(arr[n - 1 - i as usize]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1471_example_1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 2;

        let result = vec![5, 1];

        assert_eq!(Solution::get_strongest(arr, k), result);
    }

    #[test]
    fn test_1471_example_2() {
        let arr = vec![1, 1, 3, 5, 5];
        let k = 2;

        let result = vec![5, 5];

        assert_eq!(Solution::get_strongest(arr, k), result);
    }

    #[test]
    fn test_1471_example_3() {
        let arr = vec![6, 7, 11, 7, 6, 8];
        let k = 5;

        let result = vec![11, 8, 6, 6, 7];

        assert_eq!(Solution::get_strongest(arr, k), result);
    }
}
