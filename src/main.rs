mod solutions;
mod tests;

fn main() {
    println!("{}", solutions::prob1190::Solution::reserve_parentheses("(abcd)".to_string()));
}