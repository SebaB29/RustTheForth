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
pub fn apply_output_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "CR" => {
            println!();
            Ok(())
        }
        "." => point(stack),
        _ => Err("Error: Operación de salida no reconocida".to_string()),
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
            print!("{} ", value);
            Ok(())
        }
        _ => Err("Error: No hay elementos en la pila para imprimir".to_string()),
    }
}
