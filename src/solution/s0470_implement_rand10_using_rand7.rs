/**
 * [0470] Implement Rand10() Using Rand7()
 *
 * Given the API rand7() that generates a uniform random integer in the range [1, 7], write a function rand10() that generates a uniform random integer in the range [1, 10]. You can only call the API rand7(), and you shouldn't call any other API. Please do not use a language's built-in random API.
 * Each test case will have one internal argument n, the number of times that your implemented function rand10() will be called while testing. Note that this is not an argument passed to rand10().
 *  
 * Example 1:
 * Input: n = 1
 * Output: [2]
 * Example 2:
 * Input: n = 2
 * Output: [2,8]
 * Example 3:
 * Input: n = 3
 * Output: [3,8,10]
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 *  
 * Follow up:
 *
 * 	What is the <a href="https://en.wikipedia.org/wiki/Expected_value" target="_blank">expected value</a> for the number of calls to rand7() function?
 * 	Could you minimize the number of calls to rand7()?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-rand10-using-rand7/
// discuss: https://leetcode.com/problems/implement-rand10-using-rand7/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

fn rand7() -> i32 {
    0
}

const POSSIBILITIES: [[i32; 7]; 7] = [
    [1, 1, 1, 1, 2, 2, 2],
    [2, 3, 3, 3, 3, 4, 4],
    [4, 4, 5, 5, 5, 5, 6],
    [6, 6, 6, 7, 7, 7, 7],
    [8, 8, 8, 8, 9, 9, 9],
    [9, 10, 10, 10, 10, 0, 0],
    [0, 0, 0, 0, 0, 0, 0],
];

impl Solution {
    pub fn rand10() -> i32 {
        match POSSIBILITIES[(rand7() - 1) as usize][(rand7() - 1) as usize] {
            0 => Self::rand10(),
            n => n,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_0470_example_1() {
        assert_eq!(true, true);
    }
}
