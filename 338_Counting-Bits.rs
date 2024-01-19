impl Solution {
    pub fn count_bits_of_val(n:i32) -> i32{
        let mut m = n;
        let mut sum = 0;
        while(m>0){
            let digit = m%2;
            if(digit != 0){
                sum +=1;
            }
            m = m/2;
        }
        sum
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = vec![];
        for x in 0..n+1{
            ans.push( Self::count_bits_of_val(x) );
        }
        ans
    }
}
