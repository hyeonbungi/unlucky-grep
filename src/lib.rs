use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("입력한 인수의 개수가 올바르지 않습니다.\n\
            [사용법] unlucky-grep [search_query] [file_path]");
        }

        let search_query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            search_query,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.search_query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let search_query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(search_query, contents)
        );
    }
}
