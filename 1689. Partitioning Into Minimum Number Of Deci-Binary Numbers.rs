/*
    3ms 64.41%
    2.26 MB 45.12%
*/


impl Solution {
    pub fn min_partitions(n: String) -> i32 {        
        n.chars().map(|c| c as i32).max().unwrap() -48
        /*
        let mut rtn =0;
        for c in n.chars(){
            rtn = std::cmp::max(c as i32 - 48,rtn);
        }
        rtn*/
    }
    
}
