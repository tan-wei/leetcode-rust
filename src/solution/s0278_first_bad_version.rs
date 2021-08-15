/**
 * [278] First Bad Version
 *
 * You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
 * Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
 * You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
 *  
 * Example 1:
 *
 * Input: n = 5, bad = 4
 * Output: 4
 * Explanation:
 * call isBadVersion(3) -> false
 * call isBadVersion(5) -> true
 * call isBadVersion(4) -> true
 * Then 4 is the first bad version.
 *
 * Example 2:
 *
 * Input: n = 1, bad = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= bad <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-bad-version/
// discuss: https://leetcode.com/problems/first-bad-version/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // let mut left = 1;
        // let mut right = n;

        // while left != right {
        //     let middle = left + (right - left) / 2;
        //     if self.isBadVersion(middle) {
        //         right = middle;
        //     } else {
        //         left = middle + 1;
        //     }
        // }

        // left // Guaranteed to be bad.
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0278_example_1() {
        assert!(true);
    }
}
