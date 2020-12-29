use std::cmp;
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        
        let mut top = Vec::new();
        let mut left = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();
        
        // calculate skyline of top/bottom
        for row in 0..rows{
            let mut max_val = grid[row][0];
            for col in 1..cols{
                if grid[row][col] > max_val{
                    max_val = grid[row][col];
                }
            }
            top.push(max_val);
        }
        
        // calculate skyline of left/right
        for col in 0..cols{
            let mut max_val = grid[0][col];
            for row in 1..rows{
                if grid[row][col] > max_val{
                    max_val = grid[row][col];
                }
            }
            left.push(max_val);
        }
        
        //find max increase
        let mut max_inc = 0;
        for row in 0..rows{
            for col in 0..cols{
                max_inc = max_inc + (cmp::min(top[col], left[row]) - grid[row][col]);
            }        
        }
        
        return max_inc        
    }
}
