use crate::stack::Stack;
use std::fs::{self, File};
use std::io::{self, Write};

/// Lee el contenido de un archivo y lo devuelve como un `String`.
/// 
/// # Argumentos
/// 
/// * `filename` - El nombre del archivo a leer.
/// 
/// # Retorno
/// 
/// Devuelve `Ok(String)` si la lectura es exitosa, o `Err(String)` si ocurre un error.
pub fn read_file(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(error_msg) => Err(format!(
            "Error al leer el archivo '{}': {}",
            filename, error_msg
        )),
    }
}

/// Guarda el contenido de la pila en un archivo, manteniendo el orden original.
/// 
/// # Argumentos
/// 
/// * `stack` - Referencia mutable a la pila de datos.
/// * `filename` - Nombre del archivo donde se guardará el contenido de la pila.
/// 
/// # Retorno
/// 
/// Devuelve `Ok(())` si la operación es exitosa, o `Err(io::Error)` si ocurre un error.
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
