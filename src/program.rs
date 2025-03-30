use crate::arithmetic_operations::apply_arithmetic_operation;
use crate::boolean_operations::apply_boolean_operation;
use crate::conditional_operations::apply_conditional_operation;
use crate::file_handling::{read_file, save_stack_to_file};
use crate::forth_basic_operations::apply_forth_operation;
use crate::output_operations::apply_output_operation;
use crate::stack::Stack;
use std::env;

const DEFAULT_STACK_SIZE: usize = 128 * 1024;

/// Ejecuta el programa especificado en el archivo.
/// 
/// # Argumentos
/// 
/// * `stack_size` - Tamaño de la pila.
/// * `filename` - Nombre del archivo que contiene el programa Forth a ejecutar.
/// 
/// # Retornos
/// 
/// Devuelve `Ok(())` si el programa se ejecutó correctamente, o un `Err` con un mensaje de error en caso contrario.
pub fn execute_program(stack_size: usize, filename: String) -> Result<(), String> {
    let mut stack = Stack::new(stack_size);
    match read_file(filename) {
        Ok(content) => {
            if let Err(error_msg) = execute_operation(&mut stack, content) {
                return Err(error_msg);
            }

            if let Err(e) = save_stack_to_file(&mut stack, "stack.fth") {
                return Err(format!("Error al guardar la pila en el archivo: {}", e));
            }
        }
        Err(error_msg) => return Err(error_msg),
    }

    Ok(())
}

/// Analiza los argumentos de la línea de comandos.
/// 
/// # Retornos
/// 
/// Devuelve un `Result` que contiene el nombre del archivo y el tamaño de la pila si la entrada es válida, 
/// o un mensaje de error si los argumentos no son adecuados.
pub fn parse_args() -> Result<(String, usize), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Error: Debes especificar un archivo .fth".to_string());
    }

    let filename = args[1].clone();
    let stack_size = parse_stack_size(&args);
    Ok((filename, stack_size))
}

/// Ejecuta las operaciones definidas.
/// 
/// # Argumentos
/// 
/// * `stack` - La pila sobre la cual se ejecutan las operaciones.
/// * `input` - El String con las operaciones a ejecutar.
/// 
/// # Retornos
/// 
/// Devuelve `Ok(())` si las operaciones se ejecutan correctamente, o un `Err` con el mensaje de error correspondiente.
pub fn execute_operation(stack: &mut Stack, input: String) -> Result<(), String> {
    let mut tokens = input.split_whitespace();

    while let Some(token) = tokens.next() {
        let token_upc = token.to_uppercase();
        let result = match token_upc.as_str() {
            "+" | "-" | "*" | "/" => apply_arithmetic_operation(stack, &token_upc),
            "=" | "<" | ">" | "AND" | "OR" | "NOT" => apply_boolean_operation(stack, &token_upc),
            "DUP" | "DROP" | "SWAP" | "OVER" | "ROT" => apply_forth_operation(stack, &token_upc),
            "CR" | "." => apply_output_operation(stack, &token_upc),
            "IF" | "THEN" => apply_conditional_operation(stack, &token_upc, &mut tokens),
            _ => default_operation(stack, &token_upc),
        };

        if let Err(error_msg) = result {
            return Err(error_msg);
        }
    }

    Ok(())
}

/// Analiza el tamaño de la pila a partir de los argumentos de la línea de comandos.
/// 
/// # Argumentos
/// 
/// * `args` - Los argumentos de la línea de comandos.
/// 
/// # Retornos
/// 
/// Devuelve el tamaño de la pila o un valor por defecto si no se encuentra en los argumentos.
fn parse_stack_size(args: &[String]) -> usize {
    for arg in args {
        if arg.starts_with("stack-size=") {
            if let Some(size_str) = arg.split('=').nth(1) {
                if let Ok(size) = size_str.parse::<usize>() {
                    return size;
                }
            }
        }
    }

    DEFAULT_STACK_SIZE
}

/// Realiza la operación por defecto cuando el token no es reconocido como un operador.
/// 
/// # Argumentos
/// 
/// * `stack` - La pila sobre la cual se ejecuta la operación.
/// * `token` - El token que representa un número a agregar a la pila.
/// 
/// # Retornos
/// 
/// Devuelve `Ok(())` si el número se agrega correctamente a la pila, o un `Err` si el token no es un número válido.
fn default_operation(stack: &mut Stack, token: &str) -> Result<(), String> {
    if let Ok(number) = token.parse::<i16>() {
        Ok(stack.push(number))
    } else {
        Err(format!("Error: Token no reconocido '{}'", token))
    }
}
