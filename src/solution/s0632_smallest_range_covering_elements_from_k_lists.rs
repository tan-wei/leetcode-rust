/**
 * [0632] Smallest Range Covering Elements from K Lists
 *
 * You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.
 * We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.
 *  
 * Example 1:
 *
 * Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
 * Output: [20,24]
 * Explanation:
 * List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
 * List 2: [0, 9, 12, 20], 20 is in range [20,24].
 * List 3: [5, 18, 22, 30], 22 is in range [20,24].
 *
 * Example 2:
 *
 * Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
 * Output: [1,1]
 *
 *  
 * Constraints:
 *
 * 	nums.length == k
 * 	1 <= k <= 3500
 * 	1 <= nums[i].length <= 50
 * 	-10^5 <= nums[i][j] <= 10^5
 * 	nums[i] is sorted in non-decreasing order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
// discuss: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/discuss/751059/Rust-translated-BinaryHeap

#[derive(Debug, Clone, Ord, Eq)]
struct Point {
    val: i32,
    group: i32,
    idx: i32,
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut heap = std::collections::BinaryHeap::<Point>::new();
        let mut max = std::i32::MIN;
        for g in 0..n {
            let val = nums[g][0];
            if val > max {
                max = val;
            }
            heap.push(Point {
                val,
                group: g as i32,
                idx: 0,
            })
        }
        let mut start = -1;
        let mut end = -1;
        let mut range = std::i32::MAX;

        while heap.len() == n {
            let mut curr = heap.pop().unwrap();
            if max - curr.val < range {
                range = max - curr.val;
                start = curr.val;
                end = max;
            }
            if curr.idx + 1 < nums[curr.group as usize].len() as i32 {
                curr.idx += 1;
                curr.val = nums[curr.group as usize][curr.idx as usize];
                if curr.val > max {
                    max = curr.val;
                }
                heap.push(curr);
            }
        }
        vec![start as i32, end as i32]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0632_example_1() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        let result = vec![20, 24];

        assert_eq!(Solution::smallest_range(nums), result);
    }

    #[test]
    fn test_0632_example_2() {
        let nums = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let result = vec![1, 1];

        assert_eq!(Solution::smallest_range(nums), result);
    }
}
