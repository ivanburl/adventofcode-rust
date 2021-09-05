use std::fs;

#[warn(unused_variables)]
fn main() {
    let filename = "src/test.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec:Vec<&str> = contents.lines().collect();
    let (n,m) = (vec.len() as i32,vec[0].len() as i32);
    let variants = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut ans:i64 = 1;

    for _i in variants {
        let (mut x,mut y,mut tmp) = (0,0,0);
        while y<n-1 {
            y += _i.1;
            x = (x+_i.0)%m;
            tmp += (vec[y as usize].chars().nth(x as usize).unwrap()=='#') as i32;
        }
        ans *= tmp as i64;
    }

    println!("Ans - {}", ans);
}