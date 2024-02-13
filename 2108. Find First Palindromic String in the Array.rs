 use std::iter::DoubleEndedIterator;
impl Solution {
    //v2
    //4ms
    //2.18MB
    /*
     pub fn is_pal(word: &String) -> bool{
        let len = word.len();
        let mut front = word[0..len].chars();
        let mut back = word[len/2..len].chars();
        loop{
            let a = match front.next(){
                Some(n) => n,
                None => return true,
            };
            let b = match back.next_back(){
                Some(n) => n,
                None => return true,
            };
            if (a != b){
                return false;
            }
        }
        return true
    }   
*/
    //v1
    //4ms 
    //2.13 MB
    
    pub fn is_pal(word: &String) -> bool{
        let mut front = word.chars();
        loop{
            let front = match iter.next(){
                Some(n) => n,
                None => return true,
            };
            let back = match iter.next_back(){
                Some(n) => n,
                None => return true,
            };
            if (front != back){
                return false;
            }
        }
        return true
    }
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words{
            if Self::is_pal(&word){
                return word
            }
        }
        return "".to_string();
    }
}
