/**
 * [0757] Set Intersection Size At Least Two
 *
 * An integer interval [a, b] (for integers a < b) is a set of all consecutive integers from a to b, including a and b.
 * Find the minimum size of a set S such that for every integer interval A in intervals, the intersection of S with A has a size of at least two.
 *  
 * Example 1:
 *
 * Input: intervals = [[1,3],[1,4],[2,5],[3,5]]
 * Output: 3
 * Explanation: Consider the set S = {2, 3, 4}.  For each interval, there are at least 2 elements from S in the interval.
 * Also, there isn't a smaller size set that fulfills the above condition.
 * Thus, we output the size of this set, which is 3.
 *
 * Example 2:
 *
 * Input: intervals = [[1,2],[2,3],[2,4],[4,5]]
 * Output: 5
 * Explanation: An example of a minimum sized set is {1, 2, 3, 4, 5}.
 *
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 3000
 * 	intervals[i].length == 2
 * 	0 <= ai < bi <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-intersection-size-at-least-two/
// discuss: https://leetcode.com/problems/set-intersection-size-at-least-two/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/set-intersection-size-at-least-two/discuss/883105/Rust-translated-BinaryHeapSort-4ms-100
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut heap = std::collections::BinaryHeap::<(i32, i32)>::new();
        // pop with right smallest then left largest first
        for v in &intervals {
            heap.push((-v[1], v[0]));
        }
        let mut result = 0;
        let mut hi = -1;
        let mut lo = -1;

        while let Some((right, left)) = heap.pop() {
            let right = -right;
            if left <= lo {
                continue;
            };
            if left > hi {
                result += 2;
                hi = right;
                lo = hi - 1;
            } else {
                result += 1;
                lo = hi;
                hi = right;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0757_example_1() {
        let intervals = vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]];
        let result = 3;

        assert_eq!(Solution::intersection_size_two(intervals), result);
    }

    #[test]
    fn test_0757_example_2() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]];
        let result = 5;

        assert_eq!(Solution::intersection_size_two(intervals), result);
    }
}
