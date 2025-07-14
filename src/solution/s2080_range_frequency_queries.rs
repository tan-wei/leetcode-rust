/**
 * [2080] Range Frequency Queries
 *
 * Design a data structure to find the frequency of a given value in a given subarray.
 * The frequency of a value in a subarray is the number of occurrences of that value in the subarray.
 * Implement the RangeFreqQuery class:
 *
 * 	RangeFreqQuery(int[] arr) Constructs an instance of the class with the given 0-indexed integer array arr.
 * 	int query(int left, int right, int value) Returns the frequency of value in the subarray arr[left...right].
 *
 * A subarray is a contiguous sequence of elements within an array. arr[left...right] denotes the subarray that contains the elements of nums between indices left and right (inclusive).
 *  
 * Example 1:
 *
 * Input
 * ["RangeFreqQuery", "query", "query"]
 * [[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
 * Output
 * [null, 1, 2]
 * Explanation
 * RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
 * rangeFreqQuery.query(1, 2, 4); // return 1. The value 4 occurs 1 time in the subarray [33, 4]
 * rangeFreqQuery.query(0, 11, 33); // return 2. The value 33 occurs 2 times in the whole array.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i], value <= 10^4
 * 	0 <= left <= right < arr.length
 * 	At most 10^5 calls will be made to query
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-frequency-queries/
// discuss: https://leetcode.com/problems/range-frequency-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct RangeFreqQuery {
    freq: std::collections::HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut freq = std::collections::HashMap::new();

        for i in 0..arr.len() {
            freq.entry(arr[i]).or_insert(vec![]).push(i as i32);
        }

        Self { freq }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        match self.freq.get(&value) {
            Some(indices) => {
                let l = match indices.binary_search(&left) {
                    Ok(i) => i,
                    Err(i) => i,
                } as i32;
                let r = match indices.binary_search(&right) {
                    // Return the point that after (to the right of) any existing entries.
                    Ok(i) => i + 1,
                    Err(i) => i,
                } as i32;
                r - l
            }
            _ => 0,
        }
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2080_example_1() {
        let mut range_freq_query =
            RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);

        assert_eq!(range_freq_query.query(1, 2, 4), 1); // return 1. The value 4 occurs 1 time in the subarray [33, 4]

        assert_eq!(range_freq_query.query(0, 11, 33), 2); // return 2. The value 33 occurs 2 times in the whole array.
    }
}
