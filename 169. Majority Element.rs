impl Solution {
    pub fn is_pal(s:String) -> bool{
        /* v1
        let reverse:String = s.chars().rev().collect();
        return reverse == s;
        */
        //v2
        let mut reverse = s.chars().rev();
        for x in s.chars(){
            if reverse.next().unwrap() != x{
                return false;
            }
        }
        return true;
    }

    pub fn count_substrings(s: String) -> i32 {
        let mut count = 0;
        for x in 0..s.len(){
            for y in x..s.len(){
                //println!("{}",s[x..y+1].to_string());
                if Self::is_pal(s[x..y+1].to_string()){
                    count+=1;
                }
            }
        }
        count
    }
}
