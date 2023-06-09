use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query:String,
    pub filepath:String,
    pub ignore_case:bool,
}
impl Config {

    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config,&'static str>{
        args.next();
        let query=match args.next() {
            Some(q) => q,
            None => return Err("No query specified!"),
        };
        let filepath=match args.next() {
            Some(q) => q,
            None => return Err("No filepath specified!"),
        };

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
pub fn search<'a>(query:& str,content:&'a str)->Vec<String>{
    let mut ans=Vec::new();
    // for line in content.lines(){
    //     if line.contains(query)
    //     {
    //         ans.push(line);
    //     }
    // }
    // ans
    let contains:Vec<&str>=content.lines().filter(|line| line.contains(query)).collect();
    for l in contains{
        let matches:Vec<_>=l.rmatch_indices(query).collect();
        let mut temp=String::from(l);
        for m in matches{
            temp.insert_str(m.0+query.len(), "\x1B[0m");
            temp.insert_str(m.0, "\x1B[7m");
        }
        ans.push(temp);
    }
    return ans;
}

pub fn search_case_insensitive<'a>(query:& str,content:&'a str)->Vec<String>{
    let mut ans: Vec<_>=Vec::new();
    // let query=query.to_lowercase();
    // for line in content.lines(){
    //     if line.to_lowercase().contains(&query)
    //     {
    //         ans.push(line);
    //     }
    // }
    // ans
    let contains:Vec<&str>=content.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect();
    for l in contains{
        let temp_l=l.to_lowercase();
        let matches:Vec<_>=temp_l.rmatch_indices(&query.to_lowercase()).collect();
        let mut temp=String::from(l);
        for m in matches{
            temp.insert_str(m.0+query.len(), "\x1B[0m");
            temp.insert_str(m.0, "\x1B[7m");
        }
        ans.push(temp);
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