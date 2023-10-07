/**
 * [1338] Reduce Array Size to The Half
 *
 * You are given an integer array arr. You can choose a set of integers and remove all the occurrences of these integers in the array.
 * Return the minimum size of the set so that at least half of the integers of the array are removed.
 *  
 * Example 1:
 *
 * Input: arr = [3,3,3,3,5,5,5,2,2,7]
 * Output: 2
 * Explanation: Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size of the old array).
 * Possible sets of size 2 are {3,5},{3,2},{5,2}.
 * Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has a size greater than half of the size of the old array.
 *
 * Example 2:
 *
 * Input: arr = [7,7,7,7,7,7]
 * Output: 1
 * Explanation: The only possible set you can choose is {7}. This will make the new array empty.
 *
 *  
 * Constraints:
 *
 * 	2 <= arr.length <= 10^5
 * 	arr.length is even.
 * 	1 <= arr[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reduce-array-size-to-the-half/
// discuss: https://leetcode.com/problems/reduce-array-size-to-the-half/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut counts = arr.iter().fold(vec![0; 100_001], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });

        counts.sort_unstable();

        let mut sum = 0;
        for (i, &c) in counts.iter().rev().enumerate() {
            sum += c;
            if sum * 2 >= arr.len() {
                return i as i32 + 1;
            }
        }
        unreachable!()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1338_example_1() {
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        let result = 2;

        assert_eq!(Solution::min_set_size(arr), result);
    }

    #[test]
    fn test_1338_example_2() {
        let arr = vec![7, 7, 7, 7, 7, 7];
        let result = 1;

        assert_eq!(Solution::min_set_size(arr), result);
    }
}
