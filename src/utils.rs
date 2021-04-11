use std::{fs::File, io::{BufReader, Read}};

pub(crate) fn bot_config() -> Result<toml::Value, Box<dyn std::error::Error>> {
    let config = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"/actix_blog.toml"))?;
    let mut buff = BufReader::new(config);
    let mut contents = String::new();
    buff.read_to_string(&mut contents)?;
    
    let value = contents.parse::<toml::Value>()?;
    Ok(value)
}
