//Not much difference between solution 1 & 2

impl Solution {
    //convert all chars to ints
    //Combine like ints
    //check for positive or negative and sum
    
    pub fn convert(c: char) -> i32{
        match c{
            'I' => 1,
            'V' =>5,
            'X' =>10,
            'L' =>50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0, //ERR
        }
    }

    //turns char list into vec of ints
    pub fn simplifyList(s: String) -> Vec<i32>{
        let mut prev:char =' ';
        let mut iter = s.chars();
        let mut vec = Vec::<i32>::new();
        for c in iter{
            let curr = Solution::convert(c);
            if (c == prev){
                *vec.last_mut().unwrap() +=curr;
            }
            else{
                vec.push(curr);
            }
            prev = c;
        }
        vec.to_vec()
    }

    //determines if the value is positive or negative
    pub fn AddOrSubtract(vec: Vec<i32>) -> i32{
        let mut sum:i32 = vec[vec.len()-1]; //Last value is skipped in iter
        for w in vec.windows(2){
            if(w[0] > w[1]){
                sum+=w[0];
            }
            else{
                sum -=w[0];
            }
        }
        sum
    }

    pub fn Solution1(s:String) -> i32{
        Solution::AddOrSubtract(Solution::simplifyList(s))
    }
    //Solution with reversed input. Verifies if windowing is slow?
    pub fn RevAddOrSubtract(vec: Vec<i32>) -> i32{
        let mut prev:i32=0;
        let mut sum:i32 = 0;
        for cur in vec{
            if (cur>prev){
                sum+=cur;
            }   
            else{
                sum-=cur;
            }
            prev=cur
        }
        sum
    }

    pub fn Solution2(s:String) -> i32{
        let s = s.chars().rev().collect::<String>();
        let vec:Vec<i32> = Solution::simplifyList(s);
        Solution::RevAddOrSubtract(vec)
    }

    pub fn roman_to_int(s: String) -> i32 {
       Solution::Solution2(s)
    }
}
