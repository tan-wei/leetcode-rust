/**
 * [275] H-Index II
 *
 * Given an array of integers citations where citations[i] is the number of citations a researcher received for their i^th paper and citations is sorted in an ascending order, return compute the researcher's h-index.
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: A scientist has an index h if h of their n papers have at least h citations each, and the other n - h papers have no more than h citations each.
 * If there are several possible values for h, the maximum one is taken as the h-index.
 * You must write an algorithm that runs in logarithmic time.
 *  
 * Example 1:
 *
 * Input: citations = [0,1,3,5,6]
 * Output: 3
 * Explanation: [0,1,3,5,6] means the researcher has 5 papers in total and each of them had received 0, 1, 3, 5, 6 citations respectively.
 * Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
 *
 * Example 2:
 *
 * Input: citations = [1,2,100]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	n == citations.length
 * 	1 <= n <= 10^5
 * 	0 <= citations[i] <= 1000
 * 	citations is sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/h-index-ii/
// discuss: https://leetcode.com/problems/h-index-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, citations.len());
        while low < high {
            let m = (low + high) / 2;
            if citations[citations.len() - m - 1] > m as i32 {
                low = m + 1
            } else {
                high = m
            }
        }
        low as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0275_example_1() {
        let citations = vec![0, 1, 3, 5, 6];
        let result = 3;

        assert_eq!(Solution::h_index(citations), result);
    }

    #[test]
    fn test_0275_example_2() {
        let citations = vec![1, 2, 100];
        let result = 2;

        assert_eq!(Solution::h_index(citations), result);
    }
}
