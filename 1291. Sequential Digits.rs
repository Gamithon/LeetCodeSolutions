impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut rtn:Vec<i32> = vec![];
        let digits_low =  low.to_string().len();
        let digits_high =  high.to_string().len();

        let mut temp_str:String = "".to_string();
        let mut temp_char:u8 = 0;
        for digits in digits_low..digits_high+1{
            //start guess as 0-9
            for x in 0..9{
                temp_char = '1' as u8 + x;
                temp_str.clear();
                //populate each digit
                for n in 0..digits{
                    temp_str.push( temp_char as char);
                    temp_char+=1;
                }
                //When value is real number (when x=10 there's issure converting back to String)
                //check in bounds and add to list if valid
                match temp_str.parse::<i32>(){
                    Ok(n) =>{
                        if n > high{
                            break   
                        }
                        else if n < low{

                        }
                        else{ //if Self::is_sequential(temp_str.clone()) {
                            rtn.push(n)
                        }
                    }
                    Er => {break;}
                }                
                println!("{}",temp_str);
            }
        }        
        rtn
    }   
    
    //first attempt checked all values and was tooooo slow
    pub fn is_sequential(binding:String) -> bool{
        let mut str = binding.chars();
        let mut last:char = str.next().unwrap();
        for c in str{
            if c as u32 != last as u32 +1{
                return false;
            }
            else{
                last = c;
            }
        }
        return true
    }
}
