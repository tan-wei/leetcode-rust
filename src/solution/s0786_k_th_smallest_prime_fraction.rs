/**
 * [0786] K-th Smallest Prime Fraction
 *
 * You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
 * For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
 * Return the k^th smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,5], k = 3
 * Output: [2,5]
 * Explanation: The fractions to be considered in sorted order are:
 * 1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
 * The third fraction is 2/5.
 *
 * Example 2:
 *
 * Input: arr = [1,7], k = 1
 * Output: [1,7]
 *
 *  
 * Constraints:
 *
 * 	2 <= arr.length <= 1000
 * 	1 <= arr[i] <= 3 * 10^4
 * 	arr[0] == 1
 * 	arr[i] is a prime number for i > 0.
 * 	All the numbers of arr are unique and sorted in strictly increasing order.
 * 	1 <= k <= arr.length * (arr.length - 1) / 2
 *
 *  
 * Follow up: Can you solve the problem with better than O(n^2) complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-smallest-prime-fraction/
// discuss: https://leetcode.com/problems/k-th-smallest-prime-fraction/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://rustgym.com/leetcode/786

struct Fraction(i32, i32, usize, usize);

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.1 * other.0).cmp(&(self.0 * other.1))
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = std::collections::BinaryHeap::<Fraction>::new();
        let n = arr.len();
        let k = k as usize;
        for i in 0..n {
            queue.push(Fraction(arr[i], arr[n - 1], i, n - 1));
        }
        for _ in 0..k - 1 {
            let f = queue.pop().unwrap();
            if f.3 - 1 > f.2 {
                queue.push(Fraction(arr[f.2], arr[f.3 - 1], f.2, f.3 - 1));
            }
        }

        let f = queue.pop().unwrap();
        vec![f.0, f.1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0786_example_1() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let result = vec![2, 5];

        assert_eq!(Solution::kth_smallest_prime_fraction(arr, k), result);
    }

    #[test]
    fn test_0786_example_2() {
        let arr = vec![1, 7];
        let k = 1;
        let result = vec![1, 7];

        assert_eq!(Solution::kth_smallest_prime_fraction(arr, k), result);
    }
}
