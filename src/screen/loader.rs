use std::{fs::File, io::Read};

pub fn load_elements() -> Result<String, String> {
    let mut file = File::open("words.txt")
        .map_err(|e| format!("Invalid file or non content in it: {}",e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Error on reading file: {}",e))?;

    Ok(content) 
} 
