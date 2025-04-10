use crate::stack::Stack;

/// Aplica una operación de salida sobre la pila.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
/// * `operator` - El operador de salida que se va a ejecutar (CR, .).
///
/// # Errores
///
/// Retorna un `Err(String)` si el operador no es reconocido.
pub fn apply_output_operation(
    stack: &mut Stack,
    operator: &str,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<(), String> {
    match operator {
        "CR" => {
            println!();
            Ok(())
        }
        "EMIT" => emit(stack),
        "." => point(stack),
        ".\"" => print_string(tokens),
        _ => Err("?".to_string()),
    }
}

/// Imprime el valor que se encuentra en la parte superior de la pila y lo elimina.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay elementos en la pila para imprimir.
fn point(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(value) => {
            print!("{}", value);
            Ok(())
        }
        _ => Err("stack-underflow".to_string()),
    }
}

/// Convierte a caracter ASCII e imprime el valor que se encuentra en la parte superior de la pila y lo elimina.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay elementos en la pila para imprimir.
fn emit(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(value) => {
            print!("{}", value as u8 as char);
            Ok(())
        }
        _ => Err("stack-underflow".to_string()),
    }
}

/// Imprime por pantalla lo que recibe en el input que se encuentra entre ." (punto y comilla doble) y " (comilla doble)
///
/// # Argumentos
///
/// * `tokens`: Iterador de los tokens restantes en la ejecución.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay elementos en la pila para imprimir.
fn print_string(tokens: &mut std::str::SplitWhitespace) -> Result<(), String> {
    let mut collected: Vec<&str> = Vec::new();
    let mut found_closing_quote = false;

    while let Some(token) = tokens.next() {
        if token.ends_with('"') {
            let trimmed = token.trim_end_matches('"');
            collected.push(trimmed);
            found_closing_quote = true;
            break;
        } else {
            collected.push(token);
        }
    }

    if !found_closing_quote {
        return Err("Error: cadena de texto sin comilla final".to_string());
    }

    let result = collected.join(" ");
    print!("{}", result);
    Ok(())
}
