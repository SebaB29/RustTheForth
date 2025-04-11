use crate::stack::Stack;
use std::fs::{self, File};
use std::io::{self, Write};

const FILE_TO_PERSIST_DATA: &str = "stack.fth";

/// Lee el contenido de un archivo y lo devuelve como un `String`.
///
/// # Parámetros
/// - `filename`: Nombre del archivo a leer.
///
/// # Retorna
///
/// - `Ok(String)` con el contenido del archivo si la lectura es exitosa, o
/// - `Err(String)` con un mensaje de error si ocurre una falla.
pub fn read_file(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(error_msg) => Err(format!(
            "Error al leer el archivo '{}': {}",
            filename, error_msg
        )),
    }
}

/// Guarda el contenido de la pila en un archivo, preservando el orden original.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorna
/// - `Ok(())` si la operación de guardado fue exitosa, o `Err(io::Error)` si ocurre un error durante la escritura.
pub fn save_stack_to_file(stack: &mut Stack) -> Result<(), io::Error> {
    let mut file = File::create(FILE_TO_PERSIST_DATA)?;
    let mut temp_vec = Vec::new();

    while let Some(value) = stack.pop() {
        temp_vec.push(value);
    }

    for value in temp_vec.iter().rev() {
        write!(file, "{} ", value)?;
    }

    Ok(())
}
