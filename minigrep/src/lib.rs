pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_to_return: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            lines_to_return.push(line);
        }
    }

    lines_to_return
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_to_return: Vec<&str> = vec![];
    let lowercase_query: String = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            lines_to_return.push(line);
        }
    }

    lines_to_return
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn one_case_insensitive_result() {
        let query = "rusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}
