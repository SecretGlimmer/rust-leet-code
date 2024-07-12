pub struct Solution {}

impl Solution {
    pub fn reserve_parentheses(s: String) -> String {
        let mut queue: Vec<char> = Vec::new();
        let mut stack: Vec<char> = Vec::new();
        let mut result: String = String::new();

        for c in s.chars() {
            if c == ')' {
                while let Some(t) = stack.pop() {
                    if t == '(' {
                        break;
                    }
                    queue.push(t);
                }
                while !queue.is_empty() {
                    let tt = queue.remove(0);
                    stack.push(tt);
                }
            } else {
                stack.push(c);
            }
        }

        while !stack.is_empty() {
            let t = stack.remove(0);
            result.push(t);
        }

        result
    }
}

