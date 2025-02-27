// @leetup=custom
// @leetup=info id=58 lang=rust slug=length-of-last-word

/*
* Given a string `s` consisting of words and spaces, return *the length of the
* last word in the string.*
* 
* A word is a maximal substring consisting of non-space characters only.
* 
* 
* Example 1:
* 
* Input: s = "Hello World"
* Output: 5
* Explanation: The last word is "World" with length 5.
* 
* Example 2:
* 
* Input: s = "   fly me   to   the moon  "
* Output: 4
* Explanation: The last word is "moon" with length 4.
* 
* Example 3:
* 
* Input: s = "luffy is still joyboy"
* Output: 6
* Explanation: The last word is "joyboy" with length 6.
* 
* 
* Constraints:
* 
* * `1 <= s.length <= 104`
* * `s` consists of only English letters and spaces `' '`.
* * There will be at least one word in `s`.
* 
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// Test comment
// Test code
// @leetup=inject:before_code_ex

// @leetup=code

// @leetup=inject:before_code
// @leetup=inject:before_code

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last()
            .map_or(0, |word| word.len() as i32)
    }
}
// @leetup=code

// @leetup=inject:after_code

struct Solution; 

fn main() {
    let input = "Hello World".to_string();
    let solution = Solution::length_of_last_word(input);
    println!("The length of the last word is: {}", solution);
}

// @leetup=inject:after_code
