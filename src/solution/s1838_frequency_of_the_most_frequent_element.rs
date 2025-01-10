/**
 * [1838] Frequency of the Most Frequent Element
 *
 * The frequency of an element is the number of times it occurs in an array.
 * You are given an integer array nums and an integer k. In one operation, you can choose an index of nums and increment the element at that index by 1.
 * Return the maximum possible frequency of an element after performing at most k operations.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,4], k = 5
 * Output: 3
 * Explanation: Increment the first element three times and the second element two times to make nums = [4,4,4].
 * 4 has a frequency of 3.
 * Example 2:
 *
 * Input: nums = [1,4,8,13], k = 5
 * Output: 2
 * Explanation: There are multiple optimal solutions:
 * - Increment the first element three times to make nums = [4,4,8,13]. 4 has a frequency of 2.
 * - Increment the second element four times to make nums = [1,8,8,13]. 8 has a frequency of 2.
 * - Increment the third element five times to make nums = [1,4,13,13]. 13 has a frequency of 2.
 *
 * Example 3:
 *
 * Input: nums = [3,9,6], k = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 * 	1 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/frequency-of-the-most-frequent-element/
// discuss: https://leetcode.com/problems/frequency-of-the-most-frequent-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/frequency-of-the-most-frequent-element/solutions/4302322/rust-counter-solution/
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let add_to_counter = |mut m: std::collections::BTreeMap<_, _>, n| {
            *m.entry(n).or_insert(0) += 1;
            m
        };
        let freqs_m = nums
            .into_iter()
            .fold(std::collections::BTreeMap::new(), add_to_counter);
        let freqs: Vec<_> = freqs_m.into_iter().collect();

        let calc_new_counts = |(i, (n, count)): (usize, &(i32, i32))| {
            let f = |k: &mut i32, &(n_, count_): &(i32, i32)| {
                let incremented = count_.min(*k / (n - n_));
                *k -= (n - n_) * incremented;
                (incremented > 0).then(|| incremented)
            };
            freqs.iter().take(i).rev().scan(k, f).sum::<i32>() + count
        };
        freqs.iter().enumerate().map(calc_new_counts).max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1838_example_1() {
        let nums = vec![1, 2, 4];
        let k = 5;

        let result = 3;

        assert_eq!(Solution::max_frequency(nums, k), result);
    }

    #[test]
    fn test_1838_example_2() {
        let nums = vec![1, 4, 8, 13];
        let k = 5;

        let result = 2;

        assert_eq!(Solution::max_frequency(nums, k), result);
    }

    #[test]
    fn test_1838_example_3() {
        let nums = vec![3, 9, 6];
        let k = 2;

        let result = 1;

        assert_eq!(Solution::max_frequency(nums, k), result);
    }
}
