/**
 * [2053] Kth Distinct String in an Array
 *
 * A distinct string is a string that is present only once in an array.
 * Given an array of strings arr, and an integer k, return the k^th distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".
 * Note that the strings are considered in the order in which they appear in the array.
 *  
 * Example 1:
 *
 * Input: arr = ["d","b","c","b","c","a"], k = 2
 * Output: "a"
 * Explanation:
 * The only distinct strings in arr are "d" and "a".
 * "d" appears 1^st, so it is the 1^st distinct string.
 * "a" appears 2^nd, so it is the 2^nd distinct string.
 * Since k == 2, "a" is returned.
 *
 * Example 2:
 *
 * Input: arr = ["aaa","aa","a"], k = 1
 * Output: "aaa"
 * Explanation:
 * All strings in arr are distinct, so the 1^st string "aaa" is returned.
 *
 * Example 3:
 *
 * Input: arr = ["a","b","a"], k = 3
 * Output: ""
 * Explanation:
 * The only distinct string is "b". Since there are fewer than 3 distinct strings, we return an empty string "".
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= arr.length <= 1000
 * 	1 <= arr[i].length <= 5
 * 	arr[i] consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-distinct-string-in-an-array/
// discuss: https://leetcode.com/problems/kth-distinct-string-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut k = k;

        arr.iter()
            .fold(
                (
                    "".to_string(),
                    arr.iter()
                        .fold(std::collections::HashMap::new(), |mut m, x| {
                            m.entry(x).and_modify(|v| *v += 1).or_insert(1);
                            m
                        }),
                ),
                |(ans, m), x| {
                    if Some(&1) == m.get(x) {
                        k -= 1;
                        if k == 0 {
                            (x.clone(), m)
                        } else {
                            (ans, m)
                        }
                    } else {
                        (ans, m)
                    }
                },
            )
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2053_example_1() {
        let arr = vec_string!["d", "b", "c", "b", "c", "a"];
        let k = 2;

        let result = "a".to_string();

        assert_eq!(Solution::kth_distinct(arr, k), result);
    }

    #[test]
    fn test_2053_example_2() {
        let arr = vec_string!["aaa", "aa", "a"];
        let k = 1;

        let result = "aaa".to_string();

        assert_eq!(Solution::kth_distinct(arr, k), result);
    }

    #[test]
    fn test_2053_example_3() {
        let arr = vec_string!["a", "b", "a"];
        let k = 3;

        let result = "".to_string();

        assert_eq!(Solution::kth_distinct(arr, k), result);
    }
}
