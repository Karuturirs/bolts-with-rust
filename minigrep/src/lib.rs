use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{ query, filename , case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)
                    .expect("Something went wrong when reading the file.");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()

    /* //general way of implemetation for above line
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
    */
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn config_creation_positive_test(){
        let args = [String::from("dfdfd"),String::from("hhhh"), String::from("test.txt")];
        let conf = Config::new(args).unwrap();
        assert_eq!(conf.query, "hhhh");
        assert_eq!(conf.filename, "test.txt");
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn config_creation_error_test(){
        let args = [String::from("hhhh"), String::from("test.txt")];
        Config::new(args).unwrap();
    }

    #[test]
    fn run_positive_test(){


    }

    #[test]
    #[should_panic(expected = "Something went wrong when reading the file.")]
    fn run_negative_test(){
        let conf = Config{
                        query: "hhhh".to_string(),
                        filename: "test.txt".to_string(),
                        case_sensitive: false
                    };
        run(conf).unwrap();
    }

    #[test]
    fn one_result_no_content(){
        let query = "duct";
        let content = "";
        let x:Vec<&str> = vec![];
        assert_eq!(x, search(query,content));
    }


    #[test]
    fn one_result_has_content(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query,content));
    }

    #[test]
    fn one_result_case_insensitive() {
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
