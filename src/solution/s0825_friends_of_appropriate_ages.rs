/**
 * [0825] Friends Of Appropriate Ages
 *
 * There are n persons on a social media website. You are given an integer array ages where ages[i] is the age of the i^th person.
 * A Person x will not send a friend request to a person y (x != y) if any of the following conditions is true:
 *
 * 	age[y] <= 0.5 * age[x] + 7
 * 	age[y] > age[x]
 * 	age[y] > 100 &amp;&amp; age[x] < 100
 *
 * Otherwise, x will send a friend request to y.
 * Note that if x sends a request to y, y will not necessarily send a request to x. Also, a person will not send a friend request to themself.
 * Return the total number of friend requests made.
 *  
 * Example 1:
 *
 * Input: ages = [16,16]
 * Output: 2
 * Explanation: 2 people friend request each other.
 *
 * Example 2:
 *
 * Input: ages = [16,17,18]
 * Output: 2
 * Explanation: Friend requests are made 17 -> 16, 18 -> 17.
 *
 * Example 3:
 *
 * Input: ages = [20,30,100,110,120]
 * Output: 3
 * Explanation: Friend requests are made 110 -> 100, 120 -> 110, 120 -> 100.
 *
 *  
 * Constraints:
 *
 * 	n == ages.length
 * 	1 <= n <= 2 * 10^4
 * 	1 <= ages[i] <= 120
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/friends-of-appropriate-ages/
// discuss: https://leetcode.com/problems/friends-of-appropriate-ages/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        for age in ages {
            *hm.entry(age).or_default() += 1;
        }
        let mut result = 0;
        for (&a, v) in &hm {
            for (&b, u) in &hm {
                if !(b > a || 2 * b <= a + 14) {
                    result += v * u;
                    if a == b {
                        result -= v;
                    }
                }
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
    fn test_0825_example_1() {
        let ages = vec![16, 16];
        let result = 2;

        assert_eq!(Solution::num_friend_requests(ages), result);
    }

    #[test]
    fn test_0825_example_2() {
        let ages = vec![16, 17, 18];
        let result = 2;

        assert_eq!(Solution::num_friend_requests(ages), result);
    }

    #[test]
    fn test_0825_example_3() {
        let ages = vec![20, 30, 100, 110, 120];
        let result = 3;

        assert_eq!(Solution::num_friend_requests(ages), result);
    }
}
