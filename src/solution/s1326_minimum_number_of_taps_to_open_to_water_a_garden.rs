/**
 * [1326] Minimum Number of Taps to Open to Water a Garden
 *
 * There is a one-dimensional garden on the x-axis. The garden starts at the point 0 and ends at the point n. (i.e., the length of the garden is n).
 * There are n + 1 taps located at points [0, 1, ..., n] in the garden.
 * Given an integer n and an integer array ranges of length n + 1 where ranges[i] (0-indexed) means the i-th tap can water the area [i - ranges[i], i + ranges[i]] if it was open.
 * Return the minimum number of taps that should be open to water the whole garden, If the garden cannot be watered return -1.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/1685_example_1.png" style="width: 525px; height: 255px;" />
 * Input: n = 5, ranges = [3,4,1,1,0,0]
 * Output: 1
 * Explanation: The tap at point 0 can cover the interval [-3,3]
 * The tap at point 1 can cover the interval [-3,5]
 * The tap at point 2 can cover the interval [1,3]
 * The tap at point 3 can cover the interval [2,4]
 * The tap at point 4 can cover the interval [4,4]
 * The tap at point 5 can cover the interval [5,5]
 * Opening Only the second tap will water the whole garden [0,5]
 *
 * Example 2:
 *
 * Input: n = 3, ranges = [0,0,0,0]
 * Output: -1
 * Explanation: Even if you activate all the four taps you cannot water the whole garden.
 *
 *
 * Constraints:
 *
 * 	1 <= n <= 10^4
 * 	ranges.length == n + 1
 * 	0 <= ranges[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
// discuss: https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/solutions/3982461/99-5-greedy-with-dynamic-dp/
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut arr = vec![0; (n + 1) as usize];

        for i in 0..ranges.len() {
            if ranges[i] == 0 {
                continue;
            }
            let left = std::cmp::max(0, i as i32 - ranges[i]) as usize;
            arr[left] = std::cmp::max(arr[left], (i as i32 + ranges[i]) as usize);
        }

        let (mut end, mut far_can_reach, mut cnt) = (0, 0, 0);
        for i in 0..=n as usize {
            if i > end {
                if far_can_reach <= end {
                    return -1;
                }
                end = far_can_reach;
                cnt += 1;
            }
            far_can_reach = std::cmp::max(far_can_reach, arr[i]);
        }

        cnt + if end < n as usize { 1 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1326_example_1() {
        let n = 5;
        let ranges = vec![3, 4, 1, 1, 0, 0];
        let result = 1;

        assert_eq!(Solution::min_taps(n, ranges), result);
    }

    #[test]
    fn test_1326_example_2() {
        let n = 3;
        let ranges = vec![0, 0, 0, 0];
        let result = -1;

        assert_eq!(Solution::min_taps(n, ranges), result);
    }
}
