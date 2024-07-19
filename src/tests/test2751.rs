#[allow(unused_imports)]
use crate::solutions::prob2751::Solution;


#[test]
fn test_survived_robots_healths() {
    assert_eq!(
        Solution::survived_robots_healths(vec![5,4,3,2,1],vec![2,17,9,15,10],"RRRRR".to_string()),
        vec![2,17,9,15,10]
    );
    assert_eq!(
        Solution::survived_robots_healths(vec![3,5,2,6],vec![10,10,15,12],"RLRL".to_string()),
        vec![14]
    );
    assert_eq!(
        Solution::survived_robots_healths(vec![1,2,5,6],vec![10,10,11,11],"RLRL".to_string()),
        vec![]
    );
}