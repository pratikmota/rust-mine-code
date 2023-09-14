use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
fn main() {
    println!("Hello, world!");
    let x: i128 = 3326780573292121781722961636304313490;
    println!("{}", x);

    let path =
        "/home/code/PROGRAMMING/RUST-LANG/rust-mine-code/mine-problem/src/input/small_input.txt"
            .to_string();
    //let dir = env::current_dir().unwrap();
    //println!("starting dir: {}", dir);
    let file_num = load_from_file(&path);
    //println!("{:?}", file_num);
    let safe_num = 5;
    let final_result = mine_result(&safe_num, &file_num);
    println!("{}", final_result);
}
fn mine_result(safe_number: &i32, input_num: &Vec<i64>) -> String {
    // if  nums.len() < safe_number {
    //     "Mine is Safe".to_string()
    //     return
    // }
    // let v = vec![1; 10];
    // for (pos, e) in v.iter().enumerate() {
    //     println!("Element at position {}: {:?}", pos, e);
    // }

    for i in input_num.iter().skip(5) {
        println! {"{:?}", i}
    }

    // for i in input_num[5..].iter() {
    //     println! {"{:?}", i}
    // }

    // for i in nums {
    //     println! {"{:?}", i}
    // }

    "hello".to_string()
}

//load_from_file is used to load file
fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let input_numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    input_numbers
}
