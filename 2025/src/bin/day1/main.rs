use std::io;
use std::fs::File;
use advent_of_code::read_input;
use std::fs::read_to_string;



fn main() {
    let input: Vec<String> = read_input("day1").unwrap();

    let mut pos = 50;
    let mut count = 0;
    for mut turn in input{
        
        let dist = turn.split_off(1).parse::<i32>().unwrap();
        
        

        if turn.chars().nth(0).unwrap() == 'R'{
            pos += dist;
            pos = pos%100;
            if dist>= 100-pos{count+=1; println!("Passed 0");}
        }else{
            pos-=dist;
            pos = pos%100;
            if dist>=pos{count+=1; println!("Passed 0");}
        }
        if pos<0 {pos+=100;}
        println!{"{}",pos};
    }
    println!("{:?}",count);


}
