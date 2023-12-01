// 0ms Runtime          (Beats 100% of users w/ rust)
// 1.98 MB of memory    (Beats 97.69% of users w/ rust)

impl Solution {
    pub fn matchbrackets(c1: char ,c2: char) -> bool{
        if(c1 == '{' && c2 == '}'){
            return true;
        }
        if (c1 == '(' && c2 == ')'){
            return true;
        }
        if (c1 == '[' && c2== ']'){
            return true;
        }
        return false;
    }

    pub fn is_valid(s: String) -> bool {
        let mut buffer:String = "".to_string();

        for c in s.chars(){
            match c{
                '(' | '{' | '[' => buffer.push(c),
                ')' | '}' | ']' => {
                    match buffer.pop(){
                        Some(b) => {
                            if( !Solution::matchbrackets(b, c )){
                                return false;
                            }
                        },
                        None => {return false;},
                    }
                }
                _ => (),
            }
        }
        if (buffer.is_empty()){
            return true;
        }
        else{
            return false;
        }
            
    }
}
