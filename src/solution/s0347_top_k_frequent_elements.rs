/**
 * [347] Top K Frequent Elements
 *
 * Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
 *  
 * Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	k is in the range [1, the number of unique elements in the array].
 * 	It is guaranteed that the answer is unique.
 *
 *  
 * Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/top-k-frequent-elements/
// discuss: https://leetcode.com/problems/top-k-frequent-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for n in nums.iter() {
            *hm.entry(*n).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        for (&k, &v) in hm.iter() {
            buckets[v as usize].push(k);
        }
        let result = buckets
            .iter()
            .map(|b| b.iter().map(|i| *i))
            .flatten()
            .collect::<Vec<i32>>();
        result[result.len() - k as usize..].to_vec()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0347_example_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = vec![1, 2];

        assert_eq_sorted!(Solution::top_k_frequent(nums, k), result);
    }

    #[test]
    fn test_0347_example_2() {
        let nums = vec![1];
        let k = 1;
        let result = vec![1];

        assert_eq_sorted!(Solution::top_k_frequent(nums, k), result);
    }
}
