use std::env;
use std::process;

fn main() {
    let config_const_values = read_toml(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("Original: {:#?}", config_const_values);
    println!(
        "[Postgresql].Database: {}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}

// refactoring code according to the rust programming langugage book
fn read_toml(mut args: env::Args) -> Result<toml::Value, &'static str> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a filename"),
    };

    let config_text = match std::fs::read_to_string(&filename) {
        Ok(text) => text,
        Err(_) => return Err("File not found"),
    };

    let toml_value = match config_text.parse::<toml::Value>() {
        Ok(val) => val,
        Err(_) => return Err("Cannot convert the value of the toml file."),
    };

    Ok(toml_value)
}
