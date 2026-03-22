/**
 * [2404] Most Frequent Even Element
 *
 * Given an integer array nums, return the most frequent even element.
 * If there is a tie, return the smallest one. If there is no such element, return -1.
 *  
 * Example 1:
 *
 * Input: nums = [0,1,2,2,4,4,1]
 * Output: 2
 * Explanation:
 * The even elements are 0, 2, and 4. Of these, 2 and 4 appear the most.
 * We return the smallest one, which is 2.
 * Example 2:
 *
 * Input: nums = [4,4,4,9,2,4]
 * Output: 4
 * Explanation: 4 is the even element appears the most.
 *
 * Example 3:
 *
 * Input: nums = [29,47,21,41,13,37,25,7]
 * Output: -1
 * Explanation: There is no even element.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2000
 * 	0 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/most-frequent-even-element/
// discuss: https://leetcode.com/problems/most-frequent-even-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|num| num % 2 == 0)
            .fold(
                (std::collections::HashMap::new(), -1, 0),
                |(mut map, mut el, mut cnt), num| {
                    let mut entry = map.entry(num).or_insert(0);
                    *entry += 1;
                    if *entry == cnt && num < el || *entry > cnt {
                        el = num;
                        cnt = *entry;
                    }
                    (map, el, cnt)
                },
            )
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2404_example_1() {
        let nums = vec![0, 1, 2, 2, 4, 4, 1];

        let result = 2;

        assert_eq!(Solution::most_frequent_even(nums), result);
    }

    #[test]
    fn test_2404_example_2() {
        let nums = vec![4, 4, 4, 9, 2, 4];

        let result = 4;

        assert_eq!(Solution::most_frequent_even(nums), result);
    }

    #[test]
    fn test_2404_example_3() {
        let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];

        let result = -1;

        assert_eq!(Solution::most_frequent_even(nums), result);
    }
}
