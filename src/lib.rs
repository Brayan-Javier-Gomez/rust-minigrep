use std::{error::Error, fs,env};

pub struct Config {
    query: String,
    filepath: String,
    ignore_case:bool
}

impl Config {
    pub fn parse_configs(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg)=>arg,
            None => return Err("No se ha obtenido una cadena de texto para busqueda.")
        };
        let filepath = match args.next() {
            Some(arg)=>arg,
            None => return Err("No se ha podido obtener la ruta del archivo.")
        };
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
    contents.lines().filter(|line| line.contains(query)).collect()
}

//search with case sensitive
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(query.to_string().to_lowercase().as_str())).collect()
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