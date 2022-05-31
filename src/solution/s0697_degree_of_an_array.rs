/**
 * [0697] Degree of an Array
 *
 * Given a non-empty array of non-negative integers nums, the degree of this array is defined as the maximum frequency of any one of its elements.
 * Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the same degree as nums.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,2,3,1]
 * Output: 2
 * Explanation:
 * The input array has a degree of 2 because both elements 1 and 2 appear twice.
 * Of the subarrays that have the same degree:
 * [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
 * The shortest length is 2. So return 2.
 *
 * Example 2:
 *
 * Input: nums = [1,2,2,3,1,4,2]
 * Output: 6
 * Explanation:
 * The degree is 3 because the element 2 is repeated 3 times.
 * So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.
 *
 *  
 * Constraints:
 *
 * 	nums.length will be between 1 and 50,000.
 * 	nums[i] will be an integer between 0 and 49,999.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/degree-of-an-array/
// discuss: https://leetcode.com/problems/degree-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        nums.iter().enumerate().for_each(|(i, n)| {
            map.entry(*n)
                .and_modify(|v: &mut [usize; 3]| {
                    v[0] += 1;
                    v[2] = i;
                })
                .or_insert([1, i, i]);
        });

        let max_entry = map
            .iter()
            .max_by_key(|(_k, v)| (v[0], -1 * ((v[2] - v[1]) as i32)));

        if let Some((_k, val)) = max_entry {
            (val[2] - val[1] + 1) as i32
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0697_example_1() {
        let nums = vec![1, 2, 2, 3, 1];
        let result = 2;

        assert_eq!(Solution::find_shortest_sub_array(nums), result);
    }

    #[test]
    fn test_0697_example_2() {
        let nums = vec![1, 2, 2, 3, 1, 4, 2];
        let result = 6;

        assert_eq!(Solution::find_shortest_sub_array(nums), result);
    }
}
