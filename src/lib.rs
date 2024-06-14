use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("입력한 인수의 개수가 올바르지 않습니다.\n\
            [사용법] unlucky-grep [search_query] [file_path]");
        }

        let search_query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            search_query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_query, &contents)
    } else {
        search(&config.search_query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// 1. 내용물의 . 각라인에 대해 반복한다.
// 2. 해당 라인이 질의 문자열을 담고 있는지 검사한다.
// 3. 만일 그렇다면, 반환하고자 하는 값의 리스트에 추가한다.
// 4. 아니라면 아무것도 안 한다.
// 5. 매칭된 결과 리스트를 반환한다.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let search_query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(search_query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let search_query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(search_query, contents)
        );
    }
}
