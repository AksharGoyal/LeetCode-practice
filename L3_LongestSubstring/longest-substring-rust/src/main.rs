use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes(); // for efficient iteration
        let mut max_len: usize = 0; // Will be used to track longest substring's length
        let mut pos: HashMap<&u8, usize> = HashMap::new(); // to store occurence of characters
        let mut start: usize = 0; // to keep track of sliding window's position

        for (idx, ch) in s.iter().enumerate() {
            /* Let's take an example for explanation. Suppose we have 'abcabcabc'.
            We will go (0,a), (1,b), (2,c), (3,a) and so on in iterations.
            As pos is currently empty start will stay 0. max_len will be (0-0+1=1),
            which makes sense as that would be the minimum longest substring for strings such as 'bbbb'.
            pos['a'] will be 1 as sliding window's starting position.
            Similarly, we get pos['b'] = 2 and pos['c'] = 3. max_len is now 3.
            When we have idx=3 and ch='a', start will now become 3 since pos has a value for 'a'.
            max_len will be max(3, 3-3+1) which is still 3. pos['a'] will now become 4.
            Similarly, we will have pos['b']=5 and pos['c']=6 to decide the starting position of sliding window.
            Unless there is a longer substring with no repeating characters, max_len will remain 3 in this example.
            */
            if let Some(&index) = pos.get(&ch) {
                start = start.max(index);
            }
            max_len = max_len.max(idx - start + 1);
            pos.insert(ch, idx + 1);
        }
        return max_len as i32;
    }
}

#[test]
fn test_empty_string() {
    assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
    // assert_ne!(Solution::length_of_longest_substring(String::from(" ")), 0);
}

#[test]
fn test_mono_letter() {
    assert_eq!(Solution::length_of_longest_substring(String::from("@@@@@@@@@")), 1);
    // assert_ne!(Solution::length_of_longest_substring(String::from(" ")), 0);
}

#[test]
fn test() {
    assert_eq!(Solution::length_of_longest_substring(String::from("mississippiriver")), 4);
    // assert_ne!(Solution::length_of_longest_substring(String::from(" ")), 0);
}

fn main() {
    println!("Solution and Test for Longest Substring");
}