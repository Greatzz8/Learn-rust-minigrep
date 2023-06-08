use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub filepath:String,
}
impl Config {
    pub fn new(args:&Vec<String>) -> Config{
        if args.len()<3{
            panic!("Not enough args!");
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        Config { query: query, filepath: filepath }
    }

    pub fn build(args:&Vec<String>) -> Result<Config,&'static str>{
        if args.len()<3{
            return Err("Not enough args!");
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        Ok(Config { query: query, filepath: filepath })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content=fs::read_to_string(config.filepath)?;
    for line in search(&config.query, &content){
        println!("{}",line);
    }
    Ok(())
}
pub fn search<'a>(query:& str,content:&'a str)->Vec<&'a str>{
    let mut ans=Vec::new();
    for line in content.lines(){
        if line.contains(query)
        {
            ans.push(line);
        }
    }
    ans
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query="duct";
        let content="\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(search(query, content),vec!["safe, fast, productive."]);
    }
}