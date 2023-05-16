/**
 * [1093] Statistics from a Large Sample
 *
 * You are given a large sample of integers in the range [0, 255]. Since the sample is so large, it is represented by an array count where count[k] is the number of times that k appears in the sample.
 * Calculate the following statistics:
 *
 * 	minimum: The minimum element in the sample.
 * 	maximum: The maximum element in the sample.
 * 	mean: The average of the sample, calculated as the total sum of all elements divided by the total number of elements.
 * 	median:
 *
 * 		If the sample has an odd number of elements, then the median is the middle element once the sample is sorted.
 * 		If the sample has an even number of elements, then the median is the average of the two middle elements once the sample is sorted.
 *
 *
 * 	mode: The number that appears the most in the sample. It is guaranteed to be unique.
 *
 * Return the statistics of the sample as an array of floating-point numbers [minimum, maximum, mean, median, mode]. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 *
 * Input: count = [0,1,3,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
 * Output: [1.00000,3.00000,2.37500,2.50000,3.00000]
 * Explanation: The sample represented by count is [1,2,2,2,3,3,3,3].
 * The minimum and maximum are 1 and 3 respectively.
 * The mean is (1+2+2+2+3+3+3+3) / 8 = 19 / 8 = 2.375.
 * Since the size of the sample is even, the median is the average of the two middle elements 2 and 3, which is 2.5.
 * The mode is 3 as it appears the most in the sample.
 *
 * Example 2:
 *
 * Input: count = [0,4,3,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
 * Output: [1.00000,4.00000,2.18182,2.00000,1.00000]
 * Explanation: The sample represented by count is [1,1,1,1,2,2,2,3,3,4,4].
 * The minimum and maximum are 1 and 4 respectively.
 * The mean is (1+1+1+1+2+2+2+3+3+4+4) / 11 = 24 / 11 = 2.18181818... (for display purposes, the output shows the rounded number 2.18182).
 * Since the size of the sample is odd, the median is the middle element 2.
 * The mode is 1 as it appears the most in the sample.
 *
 *  
 * Constraints:
 *
 * 	count.length == 256
 * 	0 <= count[i] <= 10^9
 * 	1 <= sum(count) <= 10^9
 * 	The mode of the sample that count represents is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/statistics-from-a-large-sample/
// discuss: https://leetcode.com/problems/statistics-from-a-large-sample/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/statistics-from-a-large-sample/solutions/755642/rust-translated/
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut l = 0;
        let mut r = 255;
        let mut nl = 0;
        let mut nr = 0;
        let mut min = 255.0;
        let mut max = -1.0;
        let mut mid1 = 0.0;
        let mut mid2 = 0.0;
        let mut mode = 0;
        let mut avg = 0.0;
        let mut mid = 0.0;
        while l <= r {
            while count[l] == 0 {
                l += 1;
            }

            while count[r] == 0 {
                r -= 1;
            }

            if nl < nr {
                avg += count[l] as f64 * l as f64;
                nl += count[l];
                if count[l] > count[mode] {
                    mode = l;
                }
                if min > l as f64 {
                    min = l as f64
                };

                mid1 = l as f64;
                l += 1;
            } else {
                avg += count[r] as f64 * r as f64;
                nr += count[r];
                if count[r] > count[mode] {
                    mode = r;
                }
                if max < r as f64 {
                    max = r as f64
                };

                mid2 = r as f64;
                r -= 1;
            }
        }

        avg /= (nl + nr) as f64;

        if nl < nr {
            mid = mid2;
        } else if nl > nr {
            mid = mid1;
        } else {
            mid = (mid1 + mid2) / 2.0;
        }

        vec![min, max, avg, mid, mode as f64]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1093_example_1() {
        let count = vec![
            0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = vec![1.00000, 3.00000, 2.37500, 2.50000, 3.00000];

        assert_eq!(Solution::sample_stats(count), result);
    }

    #[test]
    #[ignore]
    fn test_1093_example_2() {
        let count = vec![
            0, 4, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = vec![1.00000, 4.00000, 2.18182, 2.00000, 1.00000];

        assert_eq!(Solution::sample_stats(count), result);
    }
}
