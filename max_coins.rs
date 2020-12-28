impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        let mut max_coins = 0;
        let mut Bob_index = 0;
        let mut my_index = piles.len() - 2;
        
        while my_index > Bob_index {
            max_coins += piles[my_index];
            if my_index == 1{
                return max_coins
            }          
            my_index -= 2;
            Bob_index += 1;
        }
        
        max_coins
        
    }
}
