/**
 * [1090] Largest Values From Labels
 *
 * There is a set of n items. You are given two integer arrays values and labels where the value and the label of the i^th element are values[i] and labels[i] respectively. You are also given two integers numWanted and useLimit.
 * Choose a subset s of the n elements such that:
 *
 * 	The size of the subset s is less than or equal to numWanted.
 * 	There are at most useLimit items with the same label in s.
 *
 * The score of a subset is the sum of the values in the subset.
 * Return the maximum score of a subset s.
 *  
 * Example 1:
 *
 * Input: values = [5,4,3,2,1]; labels = [1,1,2,2,3], numWanted = 3, useLimit = 1
 * Output: 9
 * Explanation: The subset chosen is the first, third, and fifth items.
 *
 * Example 2:
 *
 * Input: values = [5,4,3,2,1], labels = [1,3,3,3,2], numWanted = 3, useLimit = 2
 * Output: 12
 * Explanation: The subset chosen is the first, second, and third items.
 *
 * Example 3:
 *
 * Input: values = [9,8,8,7,6], labels = [0,0,0,1,1], numWanted = 3, useLimit = 1
 * Output: 16
 * Explanation: The subset chosen is the first and fourth items.
 *
 *  
 * Constraints:
 *
 * 	n == values.length == labels.length
 * 	1 <= n <= 2 * 10^4
 * 	0 <= values[i], labels[i] <= 2 * 10^4
 * 	1 <= numWanted, useLimit <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-values-from-labels/
// discuss: https://leetcode.com/problems/largest-values-from-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut limit_counter = std::collections::HashMap::new();
        let mut data = std::collections::BinaryHeap::new();
        let mut num_wanted = num_wanted;

        for i in 0..values.len() {
            limit_counter.insert(labels[i], 0);
            data.push((values[i], labels[i]));
        }

        let mut sum = 0;
        while let Some((value, label)) = data.pop() {
            let limit_counter_val = limit_counter.get_mut(&label).unwrap();

            if num_wanted > 0 && *limit_counter_val < use_limit {
                *limit_counter_val += 1;
                num_wanted -= 1;

                sum += value;
            }
        }

        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1090_example_1() {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 1, 2, 2, 3];
        let num_wanted = 3;
        let use_limit = 1;
        let result = 9;

        assert_eq!(
            Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
            result
        );
    }
}
