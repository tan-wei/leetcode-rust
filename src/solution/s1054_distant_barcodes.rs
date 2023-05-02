/**
 * [1054] Distant Barcodes
 *
 * In a warehouse, there is a row of barcodes, where the i^th barcode is barcodes[i].
 * Rearrange the barcodes so that no two adjacent barcodes are equal. You may return any answer, and it is guaranteed an answer exists.
 *  
 * Example 1:
 * Input: barcodes = [1,1,1,2,2,2]
 * Output: [2,1,2,1,2,1]
 * Example 2:
 * Input: barcodes = [1,1,1,1,2,2,3,3]
 * Output: [1,3,1,3,1,2,1,2]
 *  
 * Constraints:
 *
 * 	1 <= barcodes.length <= 10000
 * 	1 <= barcodes[i] <= 10000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distant-barcodes/
// discuss: https://leetcode.com/problems/distant-barcodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let (mut most_k, mut most_v, mut counter) = (-1, 0, std::collections::HashMap::new());
        for b in &barcodes {
            let count = counter.entry(*b).or_insert(0);
            *count += 1;
            if *count > most_v {
                most_k = *b;
                most_v = *count;
            }
        }

        let (len, even) = (barcodes.len(), (barcodes.len() + 1) % 2);
        let (mut result, mut index) = (barcodes.clone(), 0);
        let mut push = |n, t| {
            for _ in 0..n {
                result[index] = t;
                index += 2;
                if index >= len {
                    index -= len - even;
                }
            }
        };

        push(most_v, most_k);
        for (k, v) in counter.iter() {
            if *k != most_k {
                push(*v, *k);
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
    fn test_1054_example_1() {
        let barcodes = vec![1, 1, 1, 2, 2, 2];
        let result = Solution::rearrange_barcodes(barcodes);

        for i in 1..result.len() {
            assert_ne!(result[i - 1], result[i])
        }
    }

    #[test]
    fn test_1054_example_2() {
        let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
        let result = Solution::rearrange_barcodes(barcodes);

        for i in 1..result.len() {
            assert_ne!(result[i - 1], result[i])
        }
    }
}
