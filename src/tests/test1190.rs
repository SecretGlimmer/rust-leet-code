#[allow(unused_imports)]
use crate::solutions::prob1190::Solution;


#[test]
fn test_reserve_parentheses() {
    assert_eq!(
        Solution::reserve_parentheses("(abcd)".to_string()),
        "dcba".to_string()
    );
    assert_eq!(
        Solution::reserve_parentheses("(u(love)i)".to_string()),
        "iloveu".to_string()
    );
    assert_eq!(
        Solution::reserve_parentheses("(u(love))(i)".to_string()),
        "loveui".to_string()
    );
}