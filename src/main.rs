use std::env;
use std::fs;
fn main() {
    let args:Vec<String>=env::args().collect();
    let query=&args[1];
    let filepath=&args[2];
    println!("Searching {query} in {filepath}");
    let content=fs::read_to_string(filepath).expect("Can not read the file!");
    for line in content.lines(){
        if line.contains(query)
        {
            println!("{}",line);
        }
    }
}
