/**
 * [1354] Construct Target Array With Multiple Sums
 *
 * You are given an array target of n integers. From a starting array arr consisting of n 1's, you may perform the following procedure :
 *
 * 	let x be the sum of all elements currently in your array.
 * 	choose index i, such that 0 <= i < n and set the value of arr at index i to x.
 * 	You may repeat this procedure as many times as needed.
 *
 * Return true if it is possible to construct the target array from arr, otherwise, return false.
 *  
 * Example 1:
 *
 * Input: target = [9,3,5]
 * Output: true
 * Explanation: Start with arr = [1, 1, 1]
 * [1, 1, 1], sum = 3 choose index 1
 * [1, 3, 1], sum = 5 choose index 2
 * [1, 3, 5], sum = 9 choose index 0
 * [9, 3, 5] Done
 *
 * Example 2:
 *
 * Input: target = [1,1,1,2]
 * Output: false
 * Explanation: Impossible to create target array from [1,1,1,1].
 *
 * Example 3:
 *
 * Input: target = [8,5]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	n == target.length
 * 	1 <= n <= 5 * 10^4
 * 	1 <= target[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-target-array-with-multiple-sums/
// discuss: https://leetcode.com/problems/construct-target-array-with-multiple-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/3103824/just-a-runnable-solution/
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut heap = std::collections::BinaryHeap::from(target);
        let mut others = -heap.peek().unwrap();
        for &x in heap.iter() {
            match others.checked_add(x) {
                Some(o) => others = o,
                None => return false,
            };
        }
        while heap.peek() != Some(&1) {
            let max = heap.pop().unwrap();
            if others >= max {
                return false;
            }
            let before = (max - 1) % others + 1;
            heap.push(before);
            others = others + before - heap.peek().unwrap();
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1354_example_1() {
        let target = vec![9, 3, 5];

        let result = true;

        assert_eq!(Solution::is_possible(target), result);
    }

    #[test]
    fn test_1354_example_2() {
        let target = vec![1, 1, 1, 2];

        let result = false;

        assert_eq!(Solution::is_possible(target), result);
    }

    #[test]
    fn test_1354_example_3() {
        let target = vec![8, 5];

        let result = true;

        assert_eq!(Solution::is_possible(target), result);
    }
}
