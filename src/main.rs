mod solutions;
mod tests;

fn main() {
    println!("{}", solutions::prob1190::Solution::reserve_parentheses("(abcd)".to_string()));
    println!("{}", solutions::prob1717::Solution::maximum_gain("aabbaaxybbaabb".to_string(),4,5));
}