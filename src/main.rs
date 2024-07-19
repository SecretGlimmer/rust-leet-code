mod solutions;
mod tests;

fn main() {
    // let result = solutions::prob1190::Solution::reserve_parentheses("(abcd)".to_string());
    // println!("{}", result);

    // let result = solutions::prob1717::Solution::maximum_gain("aabbaaxybbaabb".to_string(),4,5);
    // println!("{}", result);

    // let result = solutions::prob2196::Solution::create_binary_tree(vec![vec![20,15,1],vec![20,17,0],vec![50,20,1],vec![50,80,0],vec![80,19,1]]);
    // println!("{:?}", result);

    // let results = solutions::prob2751::Solution::survived_robots_healths(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string());
    // println!("{:?}", results);

    let results = solutions::prob1380::Solution::lucky_numbers(vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]]);
    println!("{:?}", results);
}