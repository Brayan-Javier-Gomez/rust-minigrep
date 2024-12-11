use std::{error::Error, fs,env};

pub struct Config {
    query: String,
    filepath: String,
    ignore_case:bool
}

impl Config {
    pub fn parse_configs(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No se han proporcionado todos los argumentos");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filepath,ignore_case })
    }
}

//Readfile and return search response
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}")
    }
    Ok(())
}

//Search without case sensitive
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

//search with case sensitive
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_function() {
        let query = "duct";
        let contents = "/ Rust: \nsafe, fast, productive.\nPick three.\nDuct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sentitive_search_function() {
        let query = "rUST";
        let contents = "Rust\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust","Trust me."], search_case_insensitive(query, contents))
    }
}