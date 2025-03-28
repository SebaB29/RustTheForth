use std::fs;

pub fn read_file(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(error_msg) => Err(format!(
            "Error al leer el archivo '{}': {}",
            filename, error_msg
        )),
    }
}
