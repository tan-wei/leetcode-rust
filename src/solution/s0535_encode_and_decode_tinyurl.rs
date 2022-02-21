/**
 * [0535] Encode and Decode TinyURL
 *
 * <blockquote>Note: This is a companion problem to the <a href="https://leetcode.com/discuss/interview-question/system-design/" target="_blank">System Design</a> problem: <a href="https://leetcode.com/discuss/interview-question/124658/Design-a-URL-Shortener-(-TinyURL-)-System/" target="_blank">Design TinyURL</a>.</blockquote>
 * TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl and it returns a short URL such as http://tinyurl.com/4e9iAk. Design a class to encode a URL and decode a tiny URL.
 * There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.
 * Implement the Solution class:
 *
 * 	Solution() Initializes the object of the system.
 * 	String encode(String longUrl) Returns a tiny URL for the given longUrl.
 * 	String decode(String shortUrl) Returns the original long URL for the given shortUrl. It is guaranteed that the given shortUrl was encoded by the same object.
 *
 *  
 * Example 1:
 *
 * Input: url = "https://leetcode.com/problems/design-tinyurl"
 * Output: "https://leetcode.com/problems/design-tinyurl"
 * Explanation:
 * Solution obj = new Solution();
 * string tiny = obj.encode(url); // returns the encoded tiny url.
 * string ans = obj.decode(tiny); // returns the original url after deconding it.
 *
 *  
 * Constraints:
 *
 * 	1 <= url.length <= 10^4
 * 	url is guranteed to be a valid URL.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/encode-and-decode-tinyurl/
// discuss: https://leetcode.com/problems/encode-and-decode-tinyurl/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::hash::{Hash, Hasher};

#[derive(Default)]
struct Codec {
    dict: std::collections::HashMap<String, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Default::default()
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let hex_digest = {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            long_url.hash(&mut s);
            let s = s.finish();
            format!("{:x}", s)
        };
        self.dict.insert(hex_digest.clone(), long_url);
        format!("http://tinyurl.com/{}", hex_digest)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let hex_digest = short_url.replace("http://tinyurl.com/", "");
        if let Some(a) = self.dict.get(&hex_digest) {
            a.clone()
        } else {
            "".to_string()
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: Vec<String> = obj.decode(s);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0535_example_1() {
        let mut obj = Codec::new();
        let s: String = obj.encode("https://leetcode.com/problems/design-tinyurl".to_string());
        let ans = obj.decode(s);
    }
}
