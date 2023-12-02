use std::convert::TryInto;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target){
            Ok(n) => return n.try_into().unwrap(),  //found index
            Err(n) => return n.try_into().unwrap(), //Error containing the index where a matching element could be inserted while maintaining sorted order.
        }
        
        /*
        //linear search
        for (pos, x) in nums.iter().enumerate(){
            if(x>=&target){
                return pos.try_into().unwrap();
            }
        }
        return nums.len().try_into().unwrap();
        */
    }
}
