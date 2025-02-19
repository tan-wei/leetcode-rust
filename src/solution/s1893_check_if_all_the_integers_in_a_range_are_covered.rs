/**
 * [1893] Check if All the Integers in a Range Are Covered
 *
 * You are given a 2D integer array ranges and two integers left and right. Each ranges[i] = [starti, endi] represents an inclusive interval between starti and endi.
 * Return true if each integer in the inclusive range [left, right] is covered by at least one interval in ranges. Return false otherwise.
 * An integer x is covered by an interval ranges[i] = [starti, endi] if starti <= x <= endi.
 *  
 * Example 1:
 *
 * Input: ranges = [[1,2],[3,4],[5,6]], left = 2, right = 5
 * Output: true
 * Explanation: Every integer between 2 and 5 is covered:
 * - 2 is covered by the first range.
 * - 3 and 4 are covered by the second range.
 * - 5 is covered by the third range.
 *
 * Example 2:
 *
 * Input: ranges = [[1,10],[10,20]], left = 21, right = 21
 * Output: false
 * Explanation: 21 is not covered by any range.
 *
 *  
 * Constraints:
 *
 * 	1 <= ranges.length <= 50
 * 	1 <= starti <= endi <= 50
 * 	1 <= left <= right <= 50
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/
// discuss: https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        (left..=right).all(|num| ranges.iter().any(|v| (v[0]..=v[1]).contains(&num)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1893_example_1() {
        let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let left = 2;
        let right = 5;

        let result = true;

        assert_eq!(Solution::is_covered(ranges, left, right), result);
    }

    #[test]
    fn test_1893_example_2() {
        let ranges = vec![vec![1, 10], vec![10, 20]];
        let left = 21;
        let right = 21;

        let result = false;

        assert_eq!(Solution::is_covered(ranges, left, right), result);
    }
}
