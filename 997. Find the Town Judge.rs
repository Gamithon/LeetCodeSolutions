/*
    Runtime: 18ms 82.61%
    Memory: 2.65 MB 100%
*/

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut possible_judge:Vec<bool> = vec![true;n as usize];
        //find judge using: The town judge trusts nobody.
        for trusts in &trust{
            let truster = trusts[0];
            possible_judge[truster as usize -1] = false;
        }
        //println!("{:?}",possible_judge);

        //Verify: There is exactly one person that satisfies properties 1 and 2.
        if &possible_judge.iter().filter(|&n| *n == true).count() != &1{
            return -1;
        }

        let guess = possible_judge.iter().position(|&r| r == true).unwrap() as i32;

        //Verify: Everybody (except for the town judge) trusts the town judge.
        let mut trusts_judge:Vec<bool> = vec![false;n as usize];
        for trusts in trust{
            //trusts judge
            if trusts[1]-1 == guess{
                trusts_judge[ trusts[0] as usize -1 ] = true;
            }
        }
        //check everyone trusts judge and judge doesn't trust self
        for index in 0..n as usize{
            if trusts_judge[index] == possible_judge[index]{
                return -1;
            }
        }
        //println!("{:?}",trusts_judge);
        
        return guess+1;        
    }

    /*
    Attempted to use bitwise operators but ran into to many issues. Decided on bool vec instead. 
    Still likely possible but I'm unsure if rust supports a binary of size 1000

    use std::ops::BitAnd;
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut final_guess: i32 = 0b1111_1111;
        let mut temp_guess: i32 = 0b0000_0000;
        for person in trust{
            temp_guess = 0b0000_0000;
            for trusted in person{
                temp_guess += 1<<trusted-1;
            }
            final_guess = temp_guess.bitand(final_guess);
        }
        println!("{:b}",temp_guess);
        for index in 0..n{
            if final_guess >> n == 0b0000_0001{
                return index;
            }
        }

        if final_guess == 0b0000_0000{//|| more then one 1 in binary
            return -1;
        }
        return -1;
    }
    */
}
