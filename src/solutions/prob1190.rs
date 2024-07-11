pub struct Solution {}

impl Solution {
    pub fn reserve_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut stack = Vec::new();

        for c in s.chars() {
            if c == '(' {
                stack.push(c);
            } else if c == ')' {
                if let Some(top) = stack.pop() {
                    result.push(top);
                }
                result.push(c);
            } else {
                result.push(c);
            }
        }

        while let Some(top) = stack.pop() {
            result.push(top);
        }

        result
    }
}

