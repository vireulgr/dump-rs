
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    //let mut result = Vec::new();
    //for line in content.lines() {
    //    if line.contains(query) {
    //        result.push(line);
    //    }
    //}

    //result

    //
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    //let mut result = Vec::new();
    //for line in content.lines() {
    //    if line.to_lowercase().contains(&query) {
    //        result.push(line);
    //    }
    //}

    //result
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
        assert_eq!(search(query, content), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(search_case_insensitive(query, content), vec!["Rust:", "Trust me."]);
    }
}
