pub struct Solution {}

impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        
        let column = matrix.len();
        let mut results: Vec<i32> = Vec::new();

        for row in matrix.iter() {
            if row.is_empty() {
                continue;
            }

            let mut min_i:usize = 0;
            let mut min_v:i32 = row[0];

            for (j, v) in row.iter().enumerate() {
                if min_v > *v {
                    (min_v, min_i) = (*v, j);
                }
            }

            let mut flag:bool = true;
            for j in 0..column {
                if min_v < matrix[j][min_i] {
                    flag = false;                    
                }
            }
            if flag == true {
                results.push(min_v);                
            }
        }

        results
    }
}