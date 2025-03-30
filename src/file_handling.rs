use crate::stack::Stack;
use std::fs::{self, File};
use std::io::{self, Write};

pub fn read_file(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(error_msg) => Err(format!(
            "Error al leer el archivo '{}': {}",
            filename, error_msg
        )),
    }
}

pub fn save_stack_to_file(stack: &mut Stack, filename: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    let mut temp_vec = Vec::new();

    while let Some(value) = stack.pop() {
        temp_vec.push(value);
    }

    for value in temp_vec.iter().rev() {
        write!(file, "{} ", value)?;
    }

    Ok(())
}
