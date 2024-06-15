use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let search_query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("검색 쿼리를 입력하지 않았습니다.\n\
            [사용법] unlucky-grep [search_query] [file_path]")
            }
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("파일 경로를 입력하지 않았습니다.\n\
            [사용법] unlucky-grep [search_query] [file_path]")
            }
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
