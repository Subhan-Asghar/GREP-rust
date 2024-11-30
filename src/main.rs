use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config=Config::new(&args);
    let text = fs::read_to_string(&config.filepath).expect("Failed");

    println!("The poem is \n{}", text);
}
struct Config {
    filepath: String,
    query: String,
}
impl Config {
    fn new(arg:&[String])->Config{
        let config=Config{
            query:arg[1].clone(),
            filepath:arg[2].clone()
        };
        config
    }
}
