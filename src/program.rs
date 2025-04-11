use crate::arithmetic_operations::apply_arithmetic_operation;
use crate::boolean_operations::apply_boolean_operation;
use crate::conditional_operations::apply_conditional_operation;
use crate::file_handling::{read_file, save_stack_to_file};
use crate::forth_basic_operations::apply_forth_operation;
use crate::output_operations::apply_output_operation;
use crate::stack::Stack;
use crate::word_definitions::{WordMap, handle_word_definition};
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
    let mut word_map = WordMap::new();
    let result = match read_file(filename) {
        Ok(content) => execute_operation(&mut stack, content, &mut word_map),
        Err(error_msg) => Err(error_msg),
    };

    if let Err(e) = save_stack_to_file(&mut stack) {
        return Err(format!("Error al guardar la pila en el archivo: {}", e));
    }

    result
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
pub fn execute_operation(
    stack: &mut Stack,
    input: String,
    word_map: &mut WordMap,
) -> Result<(), String> {
    let mut tokens = input.split_whitespace();

    while let Some(token) = tokens.next() {
        let token_upc = token.to_uppercase();
        // Si es una definición de palabra
        if token_upc == ":" {
            handle_word_definition(&mut tokens, word_map)?;
            continue;
        }

        // Si es una palabra definida por el usuario
        if let Some(definition) = word_map.get(&token_upc) {
            let definition_str = definition.join(" ");
            execute_operation(stack, definition_str, word_map)?;
            continue;
        }

        let result = match token_upc.as_str() {
            "+" | "-" | "*" | "/" => apply_arithmetic_operation(stack, &token_upc),
            "=" | "<" | ">" | "AND" | "OR" | "NOT" => apply_boolean_operation(stack, &token_upc),
            "DUP" | "DROP" | "SWAP" | "OVER" | "ROT" => apply_forth_operation(stack, &token_upc),
            "CR" | "." | "EMIT" | ".\"" => apply_output_operation(stack, &token_upc, &mut tokens),
            "IF" => apply_conditional_operation(stack, &token_upc, &mut tokens, word_map),
            _ => default_operation(stack, &token_upc),
        };

        result?
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
        stack.push(number);
        Ok(())
    } else {
        Err("?".to_string())
    }
}
