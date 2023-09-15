use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // Initialize input file path and safe number
    let cwd = env::current_dir().unwrap();
    let mut path: String = String::from(cwd.to_string_lossy());
    // For Small data
    //path.push_str("/src/input/small_input.txt");
    //let safe_num: usize = 5;

    // For large data
    path.push_str("/src/input/large_input.txt");
    let safe_num: usize = 100;

    // load from file and calculate mine OK or NOK
    let file_num = load_from_file(&path);
    calculate_mine_result(&safe_num, &file_num);
}

// calculate_mine_result will calculate and display OK / NOK results of mine
fn calculate_mine_result(safe_number: &usize, input_num: &Vec<i128>) {
    if safe_number > &input_num.len() {
        println!("MINE IS SAFE - ADD MORE ELEMENTS TO CHECK");
        return;
    }
    let condition = safe_number;
    for (pos, e) in input_num.iter().enumerate() {
        if pos > (condition - 1) {
            let mut is_ok = false;
            // getting last xx value for start and end
            let start = pos - condition;
            let end = pos - 1;
            // O(n2) loop to find mine is ok or not
            for index1 in start..end {
                for index2 in start..end {
                    // skip same element
                    if index1 != index2 {
                        // check Addition is value or not
                        if (input_num[index1] + input_num[index2]) == input_num[pos] {
                            is_ok = true;
                            break;
                        }
                    }
                }
                // break from 2nd loop if result ok
                if is_ok {
                    break;
                }
            }
            // break from 1st loop if result found
            if is_ok {
                println!("OK - Element {:?} at position {}.", e, pos + 1);
            } else {
                println!(
                    "NOK - MINE CRUMBLE - Element {:?} at position {}.",
                    e,
                    pos + 1
                );
                break;
            }
        }
    }
}

//load_from_file is used to load file
fn load_from_file(file_path: &str) -> Vec<i128> {
    // open input file
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    // add input file data in vector
    let input_numbers: Vec<i128> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i128>().unwrap())
        .collect();
    input_numbers
}
