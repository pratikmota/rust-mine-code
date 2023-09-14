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

    // let path =
    //     "/home/code/PROGRAMMING/RUST-LANG/rust-mine-code/mine-problem/src/input/large_input.txt"
    //         .to_string();

    //let dir = env::current_dir().unwrap();
    //println!("starting dir: {}", dir);
    let file_num = load_from_file(&path);
    //println!("{:?}", file_num);
    let safe_num: usize = 5;
    let final_result = mine_result(&safe_num, &file_num);
    println!("{}", final_result);
}
fn mine_result(safe_number: &usize, input_num: &Vec<i128>) -> String {
    // if  nums.len() < safe_number {
    //     "Mine is Safe".to_string()
    //     return
    // }
    // let v = vec![1; 10];
    // for (pos, e) in v.iter().enumerate() {
    //     println!("Element at position {}: {:?}", pos, e);
    // }

    let condition = safe_number;
    for (pos, e) in input_num.iter().enumerate() {
        if pos > (condition - 1) {
            let mut is_ok = false;
            let start = pos - condition;
            let end = pos - 1;
            for index1 in start..end {
                for index2 in start..end {
                    if index1 != index2 {
                        if (input_num[index1] + input_num[index2]) == input_num[pos] {
                            is_ok = true;
                            break;
                        }
                    }
                }
                if is_ok {
                    break;
                }
            }
            if is_ok {
                println!("OK -Element at position {}: {:?}", pos, e);
            } else {
                println!("NOK - Element at position {}: {:?}", pos, e);
                break;
            }
        }
    }
    //println!("Element at position {}: {:?}", pos, e);

    // for i in input_num.iter().skip(5) {
    //     println! {"{:?}", i}
    // }

    // for i in input_num[5..].iter() {
    //     println! {"{:?}", i}
    // }

    // for i in nums {
    //     println! {"{:?}", i}
    // }

    "hello".to_string()
}

//load_from_file is used to load file
fn load_from_file(file_path: &str) -> Vec<i128> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let input_numbers: Vec<i128> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i128>().unwrap())
        .collect();
    input_numbers
}
