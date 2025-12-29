use std::io;
use std::fs::File;
use advent_of_code::read_input;
use std::fs::read_to_string;



fn main() {
    let input: Vec<String> = read_input("day1").unwrap();

    let mut pos = 50;
    let mut count = 0;
    for mut turn in input{
        
        let mut dist = turn.split_off(1).parse::<i32>().unwrap();
        
        

        if turn.chars().nth(0).unwrap() == 'R'{
            count += dist/100;
            dist = dist%100;
            if dist>= 100-pos{count+=1; print!("Passed 0 ");}
            pos += dist;
            pos = pos%100;
            
        }else{
            count += dist/100;
            dist = dist%100;
            if (dist>=pos) && (pos != 0) {count+=1; print!("Passed 0 ");}
            pos-=dist;
            pos = pos%100;
            
        }
        if pos<0 {pos+=100;}

        print!{"{}",pos};
        println!("");
    }
    println!("");

    println!("{:?}",count);


}
