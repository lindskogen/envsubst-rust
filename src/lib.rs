use std::collections::HashMap;

use regex::{Captures, Regex};

pub fn replace_in_str(string: &str, replace_map: &HashMap<String, String>) -> String {
    let env_var_regex: Regex = Regex::new(r"\$(\w+)|\$\{(\w+)(:?[-=+](\$?\w+|[^}]+))?\}").unwrap();
    let result = env_var_regex.replace_all(&string, |caps: &Captures| {
        if caps.get(4).is_some() {
            format!("{}", replace_map.get(&caps[2]).or(
                if caps[4].starts_with('$') {
                    replace_map.get(&caps[4][1..])
                } else {
                    None
                }
            ).unwrap_or(&caps[4].to_string()))
        } else {
            let var_lookup = caps.get(1).or(caps.get(2));

            format!("{}", var_lookup.and_then(|k| replace_map.get(k.as_str())).unwrap_or(&caps[0].to_string()))
        }
    });

    result.to_string()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::replace_in_str;

    #[test]
    fn it_replaces_simple_forms() {
        let replace_map = HashMap::from([
            ("HOME".to_owned(), "/home/user".to_owned()),
        ]);

        assert_eq!(replace_in_str("$HOME_town", &replace_map), "$HOME_town");
        assert_eq!(replace_in_str("$NOT_HOME-town", &replace_map), "$NOT_HOME-town");
        assert_eq!(replace_in_str("$HOME-town", &replace_map), "/home/user-town");
        assert_eq!(replace_in_str("${HOME}_town", &replace_map), "/home/user_town");
        assert_eq!(replace_in_str("${HOME}town", &replace_map), "/home/usertown");
    }

    #[test]
    fn it_replaces_with_fallback() {
        let replace_map = HashMap::from([
            ("HOME".to_owned(), "/home/user".to_owned()),
            ("DEFAULT".to_owned(), "some".to_owned()),
        ]);

        assert_eq!(replace_in_str("${HOME:-$DEFAULT}_town", &replace_map), "/home/user_town");
        assert_eq!(replace_in_str("${NOTHOME:-$DEFAULT}-town", &replace_map), "some-town");
        assert_eq!(replace_in_str("$HOME-town", &replace_map), "/home/user-town");
        assert_eq!(replace_in_str("${HOME:-$DEFAULT}_town", &replace_map), "/home/user_town");
        assert_eq!(replace_in_str("${NOT_HOME:-$UNKNOWN}town", &replace_map), "$UNKNOWNtown");
        assert_eq!(replace_in_str("${UNKNOWN:-https://localhost/some/path}", &replace_map), "https://localhost/some/path");
    }

    #[test]
    fn it_matches_snapshot() {
        let replace_map = HashMap::from([
            ("BAR".to_owned(), "bar".to_owned()),
        ]);

        let input = "
            foo: $BAR
            baz: ${FOO:=baz}
            env: ${ENV:-dev}
            uri: http://${BAR:=$BAZ}.com/foo
        ";

        // host: ${BAR:+localhost}

        let expected = "
            foo: bar
            baz: baz
            env: dev
            uri: http://bar.com/foo
        ";

        // host: localhost

        assert_eq!(replace_in_str(&input, &replace_map), expected);
    }


}
