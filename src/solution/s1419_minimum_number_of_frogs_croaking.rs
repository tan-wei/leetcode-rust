/**
 * [1419] Minimum Number of Frogs Croaking
 *
 * You are given the string croakOfFrogs, which represents a combination of the string "croak" from different frogs, that is, multiple frogs can croak at the same time, so multiple "croak" are mixed.
 * Return the minimum number of different frogs to finish all the croaks in the given string.
 * A valid "croak" means a frog is printing five letters 'c', 'r', 'o', 'a', and 'k' sequentially. The frogs have to print all five letters to finish a croak. If the given string is not a combination of a valid "croak" return -1.
 *  
 * Example 1:
 *
 * Input: croakOfFrogs = "croakcroak"
 * Output: 1
 * Explanation: One frog yelling "croak" twice.
 *
 * Example 2:
 *
 * Input: croakOfFrogs = "crcoakroak"
 * Output: 2
 * Explanation: The minimum number of frogs is two.
 * The first frog could yell "crcoakroak".
 * The second frog could yell later "crcoakroak".
 *
 * Example 3:
 *
 * Input: croakOfFrogs = "croakcrook"
 * Output: -1
 * Explanation: The given string is an invalid combination of "croak" from different frogs.
 *
 *  
 * Constraints:
 *
 * 	1 <= croakOfFrogs.length <= 10^5
 * 	croakOfFrogs is either 'c', 'r', 'o', 'a', or 'k'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-frogs-croaking/
// discuss: https://leetcode.com/problems/minimum-number-of-frogs-croaking/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-frogs-croaking/solutions/2914230/rust-soluiton/
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut ques = vec![std::collections::VecDeque::new(); 5];
        let s = croak_of_frogs.chars().collect::<Vec<char>>();
        let n = s.len();

        if n % 5 != 0 {
            return -1;
        }

        for i in 0..n {
            match s[i] {
                'c' => ques[0].push_back(i),
                'r' => ques[1].push_back(i),
                'o' => ques[2].push_back(i),
                'a' => ques[3].push_back(i),
                _ => ques[4].push_back(i),
            }
        }

        let mut temp = vec![0; 5];
        let mut ranges = vec![];
        let mut li = -1;
        for i in 0..n {
            let ti = i % 5;
            if ti == 0 {
                temp = vec![0; 5];
            }
            if let Some(j) = ques[ti].pop_front() {
                let j = j as i32;
                if j <= li {
                    return -1;
                }
                temp[ti] = j as usize;
                li = j;
            } else {
                return -1;
            }

            if ti == 4 {
                ranges.push((temp[0], temp[4]));
                li = 0;
            }
        }

        let mut memo = vec![(0, 0); n];
        for (a, b) in ranges {
            memo[a].0 += 1;
            memo[b].1 += 1;
        }

        let mut result = 0;
        let mut temp = 0;

        for i in 0..n {
            temp += memo[i].0;
            result = result.max(temp);
            temp -= memo[i].1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1419_example_1() {
        let croak_of_frogs = "croakcroak".to_string();

        let result = 1;

        assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), result);
    }

    #[test]
    fn test_1419_example_2() {
        let croak_of_frogs = "crcoakroak".to_string();

        let result = 2;

        assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), result);
    }

    #[test]
    fn test_1419_example_3() {
        let croak_of_frogs = "croakcrook".to_string();

        let result = -1;

        assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), result);
    }
}
