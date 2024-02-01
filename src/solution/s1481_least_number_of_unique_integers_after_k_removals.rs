/**
 * [1481] Least Number of Unique Integers after K Removals
 *
 * Given an array of integers arr and an integer k. Find the least number of unique integers after removing exactly k elements.
 *
 * <ol>
 * </ol>
 *
 *  
 * Example 1:
 *
 *
 * Input: arr = [5,5,4], k = 1
 * Output: 1
 * Explanation: Remove the single 4, only 5 is left.
 *
 * Example 2:
 *
 *
 * Input: arr = [4,3,1,1,3,3,2], k = 3
 * Output: 2
 * Explanation: Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 10^9
 * 	0 <= k <= arr.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// discuss: https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut counts_record = std::collections::HashMap::new();
        let mut k = k;

        for &element in &arr {
            *counts_record.entry(element).or_insert(0) += 1;
        }

        let mut count_array: Vec<_> = counts_record.into_iter().collect();

        count_array.sort_by(|a, b| a.1.cmp(&b.1));

        let mut i = 0;

        while k > 0 {
            let (_, freq) = count_array[i];

            if k >= freq {
                k -= freq;

                i += 1;
            } else {
                break;
            }
        }

        (count_array.len() - i) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1481_example_1() {
        let arr = vec![5, 5, 4];
        let k = 1;

        let result = 1;

        assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), result);
    }

    #[test]
    fn test_1481_example_2() {
        let arr = vec![4, 3, 1, 1, 3, 3, 2];
        let k = 3;

        let result = 2;

        assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), result);
    }
}
