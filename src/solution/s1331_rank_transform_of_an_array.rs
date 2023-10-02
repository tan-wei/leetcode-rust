/**
 * [1331] Rank Transform of an Array
 *
 * Given an array of integers arr, replace each element with its rank.
 * The rank represents how large the element is. The rank has the following rules:
 *
 * 	Rank is an integer starting from 1.
 * 	The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
 * 	Rank should be as small as possible.
 *
 *  
 * Example 1:
 *
 * Input: arr = [40,10,20,30]
 * Output: [4,1,2,3]
 * Explanation: 40 is the largest element. 10 is the smallest. 20 is the second smallest. 30 is the third smallest.
 * Example 2:
 *
 * Input: arr = [100,100,100]
 * Output: [1,1,1]
 * Explanation: Same elements share the same rank.
 *
 * Example 3:
 *
 * Input: arr = [37,12,28,9,100,56,80,5,12]
 * Output: [5,3,4,2,8,6,7,1,3]
 *
 *  
 * Constraints:
 *
 * 	0 <= arr.length <= 10^5
 * 	-10^9 <= arr[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rank-transform-of-an-array/
// discuss: https://leetcode.com/problems/rank-transform-of-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut uniques = arr
            .clone()
            .into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        uniques.sort_unstable();
        let rank_map = uniques
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, value)| (value, index as i32 + 1))
            .collect::<std::collections::HashMap<i32, i32>>();
        arr.iter()
            .map(|value| *rank_map.get(&value).unwrap())
            .collect::<Vec<i32>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1331_example_1() {
        let arr = vec![40, 10, 20, 30];
        let result = vec![4, 1, 2, 3];

        assert_eq!(Solution::array_rank_transform(arr), result);
    }

    #[test]
    fn test_1331_example_2() {
        let arr = vec![100, 100, 100];
        let result = vec![1, 1, 1];

        assert_eq!(Solution::array_rank_transform(arr), result);
    }

    #[test]
    fn test_1331_example_3() {
        let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        let result = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];

        assert_eq!(Solution::array_rank_transform(arr), result);
    }
}
