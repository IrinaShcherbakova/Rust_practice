impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut arr = Vec::new();
        for i in 1..(n+1){
            arr.push(i);
        }
        return Self::permut(0, &mut arr)
        
    }
    
    
    fn permut(index: i32, arr: &mut Vec<i32>) -> i32{
        //println!("visit {:?}, index {}", arr, index);
        let left = index as usize;
        
        if left == arr.len() { 
            println!("beautiful {:?}", arr);
            return 1 
        }
        
        let size = arr.len();
        let mut res = 0;
        
        for i in left..size{
            Self::swap(arr, i, left);
            //println!("swap: {:?}, left {}, index {}", arr, left, index);
            if (arr[left] % (index+1) == 0) || ((index+1) % arr[left] == 0){
                res += Self::permut(index+1, arr);
            }
            Self::swap(arr, i, left);       
        }
        
        res
    }
     
    fn swap(v: &mut Vec<i32>, left: usize, right: usize){
        let temp = v[left];
        v[left] = v[right];
        v[right] = temp;       
    }
}
