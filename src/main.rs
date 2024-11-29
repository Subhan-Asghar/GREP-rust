use std::env;
use std::fs;
fn main(){
    let args :Vec<String>=env::args().collect();
    let _quary=&args[1];
    let filepath=&args[2];
    let filepath2=&args[3];


    let text=fs::read_to_string(filepath).expect("Failed");
    let text2=fs::read_to_string(filepath2).expect("Failed");
    println!("The poem is \n{}",text);
    println!("Test File  \n{}",text2);




}