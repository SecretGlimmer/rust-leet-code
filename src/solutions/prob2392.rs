pub struct Solution{}

use std::collections::VecDeque; 

impl Solution {
    pub fn build_matrix( k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>, ) -> Vec<Vec<i32>> {
        // Store the topologically sorted sequences.  
        let order_rows = Self::topo_sort(row_conditions, k); 
        let order_columns = Self::topo_sort(col_conditions, k); 
        // If no topological sort exists, return an empty array.  
        if order_rows.is_empty() || order_columns.is_empty() {
            return vec![]; 
        }
        let mut matrix = vec![vec![0; k as usize]; k as usize]; 
        for i in 0..k {
            for j in 0..k { 
                // Insert values in the matrix where row and column orders match.  
                if order_rows[i as usize] == order_columns[j as usize] { 
                    matrix[i as usize][j as usize] = order_rows[i as usize]; 
                } 
            } 
        } 
        matrix 
    } 
    fn topo_sort(edges: Vec<Vec<i32>>, n: i32) -> Vec<i32> { 
        let mut adj = vec![vec![]; (n + 1) as usize]; 
        let mut in_degree = vec![0; (n + 1) as usize]; 
        // Build adjacency list and in-degree count  
        for edge in edges { 
            let u = edge[0]; 
            let v = edge[1]; 
            adj[u as usize].push(v); 
            in_degree[v as usize] += 1; 
        } 
        // Queue for nodes with no incoming edges  
        let mut queue = VecDeque::new(); 
        for i in 1..=n { 
            if in_degree[i as usize] == 0 { 
                queue.push_back(i); 
            } 
        } 
        let mut order = Vec::new(); 
        while let Some(node) = queue.pop_front() { 
            order.push(node); 
            for &neighbor in &adj[node as usize] { 
                in_degree[neighbor as usize] -= 1; 
                if in_degree[neighbor as usize] == 0 { 
                    queue.push_back(neighbor); 
                } 
            } 
        } 
        // If the order size is less than n, a cycle exists  
        if order.len() < n as usize {
            return vec![]; 
        } order 
    } 
}