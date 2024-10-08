/**
 * [1713] Minimum Operations to Make a Subsequence
 *
 * You are given an array target that consists of distinct integers and another integer array arr that can have duplicates.
 * In one operation, you can insert any integer at any position in arr. For example, if arr = [1,4,1,2], you can add 3 in the middle and make it [1,4,<u>3</u>,1,2]. Note that you can insert the integer at the very beginning or end of the array.
 * Return the minimum number of operations needed to make target a subsequence of arr.
 * A subsequence of an array is a new array generated from the original array by deleting some elements (possibly none) without changing the remaining elements' relative order. For example, [2,7,4] is a subsequence of [4,<u>2</u>,3,<u>7</u>,2,1,<u>4</u>] (the underlined elements), while [2,4,2] is not.
 *  
 * Example 1:
 *
 * Input: target = [5,1,3], arr = [9,4,2,3,4]
 * Output: 2
 * Explanation: You can add 5 and 1 in such a way that makes arr = [<u>5</u>,9,4,<u>1</u>,2,3,4], then target will be a subsequence of arr.
 *
 * Example 2:
 *
 * Input: target = [6,4,8,1,3,2], arr = [4,7,6,2,3,8,6,1]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= target.length, arr.length <= 10^5
 * 	1 <= target[i], arr[i] <= 10^9
 * 	target contains no duplicates.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-a-subsequence/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-a-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-operations-to-make-a-subsequence/solutions/5254346/beats-100-in-rust-one-line-solution-with-a-help-of-hashmap/
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let map: std::collections::HashMap<i32, i32> = target
            .iter()
            .enumerate()
            .map(|(index, item)| (*item, index as i32))
            .collect();

        target.len() as i32
            - arr
                .iter()
                .map(|item| {
                    if let Some(index) = map.get(item) {
                        *index as i32
                    } else {
                        -1
                    }
                })
                .filter(|item| *item != -1)
                .fold(Vec::new(), |mut stack, item| {
                    let idx = stack.binary_search(&item).unwrap_or_else(|item| item);
                    if idx == stack.len() {
                        stack.push(item);
                    } else {
                        stack[idx] = item;
                    }
                    stack
                })
                .len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1713_example_1() {
        let target = vec![5, 1, 3];
        let arr = vec![9, 4, 2, 3, 4];

        let result = 2;

        assert_eq!(Solution::min_operations(target, arr), result);
    }

    #[test]
    fn test_1713_example_2() {
        let target = vec![6, 4, 8, 1, 3, 2];
        let arr = vec![4, 7, 6, 2, 3, 8, 6, 1];

        let result = 3;

        assert_eq!(Solution::min_operations(target, arr), result);
    }
}
