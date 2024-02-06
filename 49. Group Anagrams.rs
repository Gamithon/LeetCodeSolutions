impl Solution {   
    // v1
    pub fn anagram(s1:&String,s2:&String) -> bool{
        if s1.len() != s2.len(){
            return false;
        }
        else{
            let mut temp1:Vec<char> = s1.to_string().chars().collect();
            temp1.sort_unstable();
            let mut temp2:Vec<char> = s2.to_string().chars().collect();
            temp2.sort_unstable();
            //println!("{},{}",temp1.clone().into_iter().collect::<String>(),temp2.clone().into_iter().collect::<String>());
            for (a,b) in temp1.iter().zip(temp2.iter()){
                if a!=b{
                    return false;
                }
            }
        }
        return true;
    }
    
    /*
    //v2 worse!?
    pub fn anagram(s1:&String,s2:&String) -> bool{
        let mut s2:Vec<char> = s2.chars().collect();//to_string();
        let len = s2.len();

        if s1.len() != len{
            return false;
        }

        let mut found = false;
        for c in s1.chars(){
            found = false;
            for n in 0..len{
                if s2[n] == c{
                    s2[n] = 0 as char;
                    found = true;
                    break
                }
            }
            if (!found){
                return false;
            }
        }
        return true;
    }
    */

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut rtn:Vec<Vec<String>> = vec![vec![]];
        let mut iter = strs.iter();
        let mut found = false;
        match iter.next(){
            Some(n) => rtn[0].push(n.to_string()),
            None => return rtn,
        }
        //println!("ran {:?}",rtn);
        for s in iter{
            found = false;
            for x in 0..rtn.len(){
                if Self::anagram(&rtn[x][0], s){
                    //println!("add {} , {}",s,rtn[x][0]);
                    rtn[x].push(s.to_string());
                    found = true;
                }
            }
            if (!found){
                    rtn.push(vec![s.clone()]);     
            }
        }
        rtn
    }
}
