#[allow(unused_imports)]
use crate::solutions::prob1717::Solution;


#[test]
fn test_maximum_gain() {
    assert_eq!(
        Solution::maximum_gain("cdbcbbaaabab".to_string(),4,5),
        19
    );
    assert_eq!(
        Solution::maximum_gain("aabbaaxybbaabb".to_string(),5,4),
        20
    );
}