/*
    0ms     100%
    21.MB   2.1MB
*/
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let len = s.len();        
        let mut count = s.chars().filter(|c| c== &'1').count();

        let mut rtn = String::with_capacity(len); //Improvement 
        for _ in 0..count-1{
            rtn.push('1');
        }
        for _ in 0..len-count{
            rtn.push('0');
        }
        rtn.push('1');
        rtn
    }
}
