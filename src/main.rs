use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args=env::args();
    let config=Config::build(args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching {} in {}",config.query,config.filepath);
    if let Err(e)=minigrep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
    
}







