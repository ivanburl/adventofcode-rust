use std::fs;
use std::collections::HashSet;

#[warn(unused_variables)]
fn main() {
    let filename = "src/test.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut hash_set:HashSet<i32> = HashSet::new();
    for _i in contents.lines() {
        let val:i32 = match _i.trim().parse() {
            Ok(_num) => _num,
            Err(_) => continue
        };

        if hash_set.contains(&(2020-val)) {
            println!("Number found numbers: {}, {}; mult - {}", val, 2020-val, val*(2020-val));
        }
        
        hash_set.insert(val);
    }
}