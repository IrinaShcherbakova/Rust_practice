impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        
        for piece in pieces.iter(){
            if !Self::contains(&arr, piece.to_vec()){
                return false;
            }          
        }
        
        return true;
        
    }
    
    
    fn contains(arr: &Vec<i32>, piece: Vec<i32>) -> bool {
        
        let first = piece[0];
        let mut index = 0;
        let N = arr.len();
        let piece_len = piece.len();
        
        //locate the start position of piece 
        while index < N && arr[index] != first {
            index += 1;
        }
        
        if index == N {
            return false;
        }
        
        for i in 0..piece_len{
            if index + i >= N {
                return false;
            }           
            if piece[i] != arr[index+i]{
                return false;
            }
        }
        
        return true;
    }
}
