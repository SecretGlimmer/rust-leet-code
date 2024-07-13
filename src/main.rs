mod solutions;
mod tests;

fn main() {
    // println!("{}", solutions::prob1190::Solution::reserve_parentheses("(abcd)".to_string()));
    // println!("{}", solutions::prob1717::Solution::maximum_gain("aabbaaxybbaabb".to_string(),4,5));
    let results = solutions::prob2751::Solution::survived_robots_healths(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string());
    println!("{:?}", results);
}