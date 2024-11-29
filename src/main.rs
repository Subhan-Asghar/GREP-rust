use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let query = &args[0];
    // let file_path = &args[1];

    println!("Searching for {query}");
    // println!("In file {file_path}");
    println!("the strung is {:?}",args);
}