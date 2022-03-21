use std::collections::HashMap;

use regex::{Captures, Regex};

pub fn replace_in_str(string: String, replace_map: &HashMap<String, String>) -> String {
    let env_var_regex: Regex = Regex::new(r"\$\{?(\w+)\}?").unwrap();
    let result = env_var_regex.replace_all(&string, |caps: &Captures| {
        format!("{}", replace_map.get(&caps[1]).unwrap_or(&caps[0].to_string()))
    });

    result.to_string()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::replace_in_str;

    #[test]
    fn it_works() {
        let replace_map = HashMap::from([
            ("HOME".to_owned(), "/home/user".to_owned())
        ]);

        assert_eq!(replace_in_str("$HOME_town $NOT_HOME-town $HOME-town ${HOME}_town ${HOME}town".to_owned(), &replace_map), "$HOME_town $NOT_HOME-town /home/user-town /home/user_town /home/usertown")
    }
}
