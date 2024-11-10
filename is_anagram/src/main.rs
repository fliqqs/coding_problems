use std::fmt;
struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        s_chars.sort();
        t_chars.sort();
        println!("{:?} {:?}",s_chars,t_chars );
        return s_chars == t_chars;
    }
}
fn main()
{
    println!("{}", Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
    println!("{}", Solution::is_anagram(String::from("jam"), String::from("jan")));
}
