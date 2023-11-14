//Slow solution
impl Solution { 

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i:usize = 0;

        for c in strs[0].chars(){
            for str in strs.clone(){
                match str.chars().nth(i){
                    Some(n) => if( n!= c) { return strs[0].chars().take(i).collect();}
                    //Catch when collection is empty
                    None => {return strs[0].chars().take(i).collect();}
                }        
                 
            }
            i+=1
        }

        return strs[0].clone();
    }
}
