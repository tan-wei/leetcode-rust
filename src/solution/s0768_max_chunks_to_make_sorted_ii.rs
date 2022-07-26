/**
 * [0768] Max Chunks To Make Sorted II
 *
 * You are given an integer array arr.
 * We split arr into some number of chunks (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.
 * Return the largest number of chunks we can make to sort the array.
 *  
 * Example 1:
 *
 * Input: arr = [5,4,3,2,1]
 * Output: 1
 * Explanation:
 * Splitting into two or more chunks will not return the required result.
 * For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
 *
 * Example 2:
 *
 * Input: arr = [2,1,3,4,4]
 * Output: 4
 * Explanation:
 * We can split into two chunks, such as [2, 1], [3, 4, 4].
 * However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 2000
 * 	0 <= arr[i] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-chunks-to-make-sorted-ii/
// discuss: https://leetcode.com/problems/max-chunks-to-make-sorted-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut result = 0;
        let mut t = arr.clone();
        t.sort_unstable();
        for i in 0..arr.len() {
            sum1 += t[i];
            sum2 += arr[i];
            if sum1 == sum2 {
                result += 1;
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
    fn test_0768_example_1() {
        let arr = vec![5, 4, 3, 2, 1];
        let result = 1;

        assert_eq!(Solution::max_chunks_to_sorted(arr), result);
    }

    #[test]
    fn test_0768_example_2() {
        let arr = vec![2, 1, 3, 4, 4];
        let result = 4;

        assert_eq!(Solution::max_chunks_to_sorted(arr), result);
    }
}
