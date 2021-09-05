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
        let mut cnt = 0;
        for _j in result[4].chars() {
            cnt += (_j==result[2].parse().unwrap()) as i32;
        }
        
        if cnt >= result[0].parse().unwrap() && cnt <= result[1].parse().unwrap() {
            ok+=1;
        }
    }
    println!("Answer -  {}", ok);

}