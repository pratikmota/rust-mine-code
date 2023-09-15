**HOW TO BUILD**
1) Go to project directory **rust-mine-code/mine-problem**
2) cargo build   ( It will build binary in target/debug folder )
3) cargo run ( It will run binary )
   
**HOW TO TEST**
1) Input files are available at - /src/input
2) Default input file is - /src/input/large_input.txt ( safe number is 100)
3) cargo run
4) It will give output where mine is crumbled
   ![image](https://github.com/pratikmota/rust-mine-code/assets/5825319/bd873526-fd0e-4c3c-8e3c-3061ec81afca)
5) Number is OK if addition of any two last safe_number is equal to number, otherwise it is NOK
6) Same if we check for small file small_input.txt ( safe_number = 5)
   ![image](https://github.com/pratikmota/rust-mine-code/assets/5825319/3f865b88-5eae-4edc-a456-f4afeb2a3da9)
7) You can change file name and safe_number from code at given location
   ```
    path.push_str("/src/input/small_input.txt");
    let safe_num: usize = 5;
   ```
