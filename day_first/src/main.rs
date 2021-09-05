use std::fs;

#[warn(unused_variables)]
fn main() {
    let filename = "src/test.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut mas:Vec<i32> = Vec::new();
    for _i in contents.lines() {
        let val:i32 = match _i.trim().parse() {
            Ok(_num) => _num,
            Err(_) => continue
        };
        mas.push(val);
    }
    mas.sort();
    
    for _i in 0..mas.len() {
        for _j in _i..mas.len() {
            for _z in _j..mas.len() {
                let val = mas[_i]+mas[_j]+mas[_z];

                if val>2020 {
                    break;
                }

                if val==2020 {
                    println!("Answer - {}",mas[_i]*mas[_j]*mas[_z])
                }
            }
        }
    }
}