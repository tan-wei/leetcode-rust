/**
 * [1726] Tuple with Same Product
 *
 * Given an array nums of distinct positive integers, return the number of tuples (a, b, c, d) such that a * b = c * d where a, b, c, and d are elements of nums, and a != b != c != d.
 *  
 * Example 1:
 *
 * Input: nums = [2,3,4,6]
 * Output: 8
 * Explanation: There are 8 valid tuples:
 * (2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
 * (3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
 *
 * Example 2:
 *
 * Input: nums = [1,2,4,5,10]
 * Output: 16
 * Explanation: There are 16 valid tuples:
 * (1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
 * (2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
 * (2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
 * (4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 10^4
 * 	All elements in nums are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/tuple-with-same-product/
// discuss: https://leetcode.com/problems/tuple-with-same-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(
                std::collections::HashMap::new(),
                |mut map, (idx1, &num1)| {
                    nums.iter()
                        .enumerate()
                        .skip(idx1 + 1)
                        .fold(map, |mut map, (idx2, &num2)| {
                            *map.entry(num1 * num2).or_insert(0) += 1;
                            map
                        })
                },
            )
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(_, count)| (count * (count - 1) / 2) * 8)
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1726_example_1() {
        let nums = vec![2, 3, 4, 6];

        let result = 8;

        assert_eq!(Solution::tuple_same_product(nums), result);
    }

    #[test]
    fn test_1726_example_2() {
        let nums = vec![1, 2, 4, 5, 10];

        let result = 16;

        assert_eq!(Solution::tuple_same_product(nums), result);
    }
}
