use std::{fs};
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {
        if args.len()<3{
            return Err("Not enough arguments");
            
        }
        let query = args[1].clone();
        let filename = args[2].clone();    
        let config = Config{query, filename};
        return Ok(config);
    }
}
pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let content = fs::read_to_string(config.filename)?;

    let result = search(&config.query,&content);
    for line in result{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &'a str,content:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search(query,content));  
    }
}