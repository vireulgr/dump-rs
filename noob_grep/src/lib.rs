pub fn search<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    content
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(search(query, content).collect(), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(search_case_insensitive(query, content).collect(), vec!["Rust:", "Trust me."]);
    }
}
