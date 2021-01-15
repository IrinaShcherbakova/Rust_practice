impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        
        people.sort();
        
        let mut min_boats = 0;
        let mut cur_load = 0;
        let mut on_board = 0;
        let mut left = 0;
        let mut right = people.len() - 1;
        
        while left <= right {
            
            if on_board == 2 || cur_load == limit {
                cur_load = 0;
                min_boats += 1; 
                on_board = 0;
            }
            
            if people[right] + cur_load <= limit {
                cur_load += people[right];
                on_board += 1;
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            
            else if people[left] + cur_load <= limit {
                cur_load += people[left];
                on_board += 1;
                left += 1;
            }
            
            else {
                cur_load = 0;
                min_boats += 1; 
                on_board = 0;
            } 
            
        }
        
        min_boats + 1
    }
}
