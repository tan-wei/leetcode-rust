/**
 * [1562] Find Latest Group of Size M
 *
 * Given an array arr that represents a permutation of numbers from 1 to n.
 * You have a binary string of size n that initially has all its bits set to zero. At each step i (assuming both the binary string and arr are 1-indexed) from 1 to n, the bit at position arr[i] is set to 1.
 * You are also given an integer m. Find the latest step at which there exists a group of ones of length m. A group of ones is a contiguous substring of 1's such that it cannot be extended in either direction.
 * Return the latest step at which there exists a group of ones of length exactly m. If no such group exists, return -1.
 *  
 * Example 1:
 *
 * Input: arr = [3,5,1,2,4], m = 1
 * Output: 4
 * Explanation:
 * Step 1: "00<u>1</u>00", groups: ["1"]
 * Step 2: "0010<u>1</u>", groups: ["1", "1"]
 * Step 3: "<u>1</u>0101", groups: ["1", "1", "1"]
 * Step 4: "1<u>1</u>101", groups: ["111", "1"]
 * Step 5: "111<u>1</u>1", groups: ["11111"]
 * The latest step at which there exists a group of size 1 is step 4.
 *
 * Example 2:
 *
 * Input: arr = [3,1,5,4,2], m = 2
 * Output: -1
 * Explanation:
 * Step 1: "00<u>1</u>00", groups: ["1"]
 * Step 2: "<u>1</u>0100", groups: ["1", "1"]
 * Step 3: "1010<u>1</u>", groups: ["1", "1", "1"]
 * Step 4: "101<u>1</u>1", groups: ["1", "111"]
 * Step 5: "1<u>1</u>111", groups: ["11111"]
 * No group of size 2 exists during any step.
 *
 *  
 * Constraints:
 *
 * 	n == arr.length
 * 	1 <= m <= n <= 10^5
 * 	1 <= arr[i] <= n
 * 	All integers in arr are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-latest-group-of-size-m/
// discuss: https://leetcode.com/problems/find-latest-group-of-size-m/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let (n, m) = (arr.len(), m as usize);
        if n == m {
            return n as i32;
        }
        let mut length = vec![0; n + 2];
        let mut count = vec![0; n + 1];
        let mut result = -1;

        for (i, &a) in arr.iter().enumerate() {
            let (a, low, high) = (a as usize, length[a as usize - 1], length[a as usize + 1]);
            let sum = low + high + 1;
            length[a - low] = sum;
            length[a + high] = sum;
            count[low] -= 1;
            count[high] -= 1;
            count[sum] += 1;
            if count[m] > 0 {
                result = i as i32 + 1;
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
    fn test_1562_example_1() {
        let arr = vec![3, 5, 1, 2, 4];
        let m = 1;

        let result = 4;

        assert_eq!(Solution::find_latest_step(arr, m), result);
    }

    #[test]
    fn test_1562_example_2() {
        let arr = vec![3, 1, 5, 4, 2];
        let m = 2;

        let result = -1;

        assert_eq!(Solution::find_latest_step(arr, m), result);
    }
}
