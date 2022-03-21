use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Read;

use envsubst::replace_in_str;

fn main() -> io::Result<()> {
    let replace_map: HashMap<String, String> = env::vars().collect();

    let mut string = String::new();

    io::stdin().read_to_string(&mut string)?;

    let result = replace_in_str(string, &replace_map);

    println!("{}", result);

    Ok(())
}



