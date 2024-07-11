use crate::solutions::prob1190::Solution;

#[test]
fn test_reserve_parentheses() {
    assert_eq!(
        Solution::reserve_parentheses("(abcd)".to_string()),
        "(dcba)".to_string()
    );
    assert_eq!(
        Solution::reserve_parentheses("(u(love)i)".to_string()),
        "(i(love)u)".to_string()
    );
    assert_eq!(
        Solution::reserve_parentheses("(u(love))(i)".to_string()),
        "(i)(u(love))".to_string()
    );
}