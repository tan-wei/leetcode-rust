/**
 * [0710] Random Pick with Blacklist
 *
 * You are given an integer n and an array of unique integers blacklist. Design an algorithm to pick a random integer in the range [0, n - 1] that is not in blacklist. Any integer that is in the mentioned range and not in blacklist should be equally likely to be returned.
 * Optimize your algorithm such that it minimizes the number of calls to the built-in random function of your language.
 * Implement the Solution class:
 *
 * 	Solution(int n, int[] blacklist) Initializes the object with the integer n and the blacklisted integers blacklist.
 * 	int pick() Returns a random integer in the range [0, n - 1] and not in blacklist.
 *
 *  
 * Example 1:
 *
 * Input
 * ["Solution", "pick", "pick", "pick", "pick", "pick", "pick", "pick"]
 * [[7, [2, 3, 5]], [], [], [], [], [], [], []]
 * Output
 * [null, 0, 4, 1, 6, 1, 0, 4]
 * Explanation
 * Solution solution = new Solution(7, [2, 3, 5]);
 * solution.pick(); // return 0, any integer from [0,1,4,6] should be ok. Note that for every call of pick,
 *                  // 0, 1, 4, and 6 must be equally likely to be returned (i.e., with probability 1/4).
 * solution.pick(); // return 4
 * solution.pick(); // return 1
 * solution.pick(); // return 6
 * solution.pick(); // return 1
 * solution.pick(); // return 0
 * solution.pick(); // return 4
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 * 	0 <= blacklist.length <- min(10^5, n - 1)
 * 	0 <= blacklist[i] < n
 * 	All the values of blacklist are unique.
 * 	At most 2 * 10^4 calls will be made to pick.
 *
 */
// problem: https://leetcode.com/problems/random-pick-with-blacklist/
// discuss: https://leetcode.com/problems/random-pick-with-blacklist/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use rand::prelude::*;
use std::collections::HashMap;

struct Solution {
    rng: ThreadRng,
    map: HashMap<i32, i32>,
    m: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let rng = Default::default();

        let mut map = std::collections::HashMap::<i32, i32>::new();
        for &b in &blacklist {
            map.insert(b, -1);
        }

        let mut n = n;
        let m = n - map.len() as i32;
        for b in blacklist {
            if b < m {
                while map.contains_key(&(n - 1)) {
                    n -= 1;
                }
                map.insert(b, n - 1);
                n -= 1;
            }
        }

        Self { rng, map, m }
    }

    fn pick(&mut self) -> i32 {
        let p = self.rng.gen_range(0, self.m);
        if self.map.contains_key(&p) {
            *self.map.get(&p).unwrap()
        } else {
            p
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0710_example_1() {
        let mut solution = Solution::new(7, vec![2, 3, 5]);
        solution.pick(); // return 0, any integer from [0,1,4,6] should be ok. Note that for every call of pick,
        // 0, 1, 4, and 6 must be equally likely to be returned (i.e., with probability 1/4).
        solution.pick(); // return 4
        solution.pick(); // return 1
        solution.pick(); // return 6
        solution.pick(); // return 1
        solution.pick(); // return 0
        solution.pick(); // return 4
    }
}
