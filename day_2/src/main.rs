use std::fs;

#[warn(unused_variables)]
fn main() {
    let filename = "src/test.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut ok = 0;
    for _i in contents.lines() {
        let result:Vec<&str>  = _i.trim().split(|c: char| c=='-' || c==':' || c==' ')
                                        .map(|str: &str| str.trim())
                                        .collect();
        
        let pos1:usize = result[0].parse().unwrap();
        let pos2:usize = result[1].parse().unwrap();
        let ch:char = result[2].parse().unwrap();
        let str =  result[4];
        let sum:i32 = match str.chars().nth(pos1-1) {
            Some(_val) => (_val==ch) as i32,
            None => 0
        } +
        match str.chars().nth(pos2-1) {
            Some(_val) => (_val==ch) as i32,
            None => 0
        };
        
        if sum==1 {
            ok+=1;
        }
    }
    println!("Answer -  {}", ok);

}