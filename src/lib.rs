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
    println!("내용:\n{}", contents);

    Ok(())
}
