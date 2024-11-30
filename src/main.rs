use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config=Config::new(&args);
    println!("The file path is {}",&config.filepath);
    println!("The query is {}",&config.query);
    run(&config);

}
fn run(config:&Config){
    let text = fs::read_to_string(&config.filepath).expect("Failed");
    let result =search(&config.query,&text);
    if result.is_empty() {
        println!("No matches found for query: {}", &config.query);
    } else {
        println!("The matching lines are:");
        for line in result {
            println!("{}", line);
        }
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
struct Config {
    filepath: String,
    query: String,
}
impl Config {
    fn new(arg:&[String])->Config{
        if arg.len()<3{
            panic!("Panic :)");
        }
        let config=Config{
            query:arg[1].clone(),
            filepath:arg[2].clone()
        };
        config
    }
}
