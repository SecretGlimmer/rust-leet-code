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


// let mut t:char = '#';
// let mut l:char = '#';
// let mut cnt:i32 = 0;
// while let Some(c) = stack.pop() {
//     if t != c && (c == 'a' || c == 'b'){
//         if t == '#' {
//             l = c
//         }
//         t = c;
//         cnt += 1;
//     } else {
//         stack.push(c);
//         break;
//     }
// }
// println!("{} {}",t, l);
// while let Some(c) = temp.pop() {
//     if l == c || (c != 'a' && c != 'b') || l == '#' {
//         temp.push(c);
//         break;
//     }
//     l = c;
//     cnt += 1;
// }
// if cnt > 1 {
//     if cnt % 2 == 0 {
//         if t == 'a' {
//             sum += (cnt/2-1)*mx+x;
//         } else if t == 'b' {
//             sum += (cnt/2-1)*mx+y;
//         }
//     } else {
//         stack.push(t);
//         sum += (cnt/2)*mx;
//     }  
// }
// if cnt == 1 {
//     temp.push(t);
// }
// if cnt == 0 {
//     if let Some(c) = stack.pop() {
//         temp.push(c);
//     }
// }
// for c in &stack {
//     print!("{}",c);
// }
// print!(" ");
// for c in &temp {
//     print!("{}",c);
// }
// println!(" {} {} {} {}",cnt, t, l, sum);        