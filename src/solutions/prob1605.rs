pub struct Solution {}

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let row = row_sum.len();
        let col = col_sum.len();
        let mut res = vec![vec![0; col]; row];

        for i in 0..row {
            for j in 0..col {
                let val = row_sum[i].min(col_sum[j]);

                res[i][j] = val;

                row_sum[i] -= val;
                col_sum[j] -= val;
            }
        }

        res
    }
}
