pub struct Solution {}

use std::cmp;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut sum:i32 = 0;
        let mut stack:Vec<char> = Vec::new();
        let mut temp:Vec<char> = Vec::new();

        for c in s.chars() {
            stack.push(c);
        }

        while !stack.is_empty() {
            while let Some(c) = stack.pop() {
                if c != 'a' && c != 'b' {
                    break;
                }
                if let Some(t) = temp.pop() {
                    if x > y && t == 'b' && c == 'a' {
                        sum += x;
                    } else if x <= y && t == 'a' && c == 'b' {
                        sum += y;
                    } else {
                        temp.push(t);
                        temp.push(c);
                    }
                } else {
                    temp.push(c);
                }
            }      
            let mut cnta:i32 = 0;
            let mut cntb:i32 = 0;
            while let Some(t) = temp.pop() {
                if t == 'a' {
                    cnta += 1;
                } else if t == 'b' {
                    cntb += 1;
                }
            }
            sum += cmp::min(cnta, cntb) * cmp::min(x,y);
        }

        sum
    }
}
