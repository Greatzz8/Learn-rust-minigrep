use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query:String,
    pub filepath:String,
    pub ignore_case:bool,
}
impl Config {
    pub fn new(args:&Vec<String>) -> Config{
        if args.len()<3{
            panic!("Not enough args!");
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        Config { query: query, filepath: filepath,ignore_case:env::var("IGNORE_CASE").is_ok() }
    }

    pub fn build(args:&Vec<String>) -> Result<Config,&'static str>{
        if args.len()<3{
            return Err("Not enough args!");
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        Ok(Config { query: query, filepath: filepath,ignore_case:env::var("IGNORE_CASE").is_ok() })
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content=fs::read_to_string(config.filepath)?;
    let result=if config.ignore_case{
        search_case_insensitive(&config.query, &content)
    }else{
        search(&config.query, &content)
    };
    for line in result{
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

pub fn search_case_insensitive<'a>(query:& str,content:&'a str)->Vec<&'a str>{
    let mut ans=Vec::new();
    let query=query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query)
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
Pick three.
Duct";
        assert_eq!(search(query, content),vec!["safe, fast, productive."]);
    }
    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}