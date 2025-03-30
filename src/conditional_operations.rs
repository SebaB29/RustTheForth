use crate::stack::Stack;

/// Aplica una operación condicional en la pila según el operador recibido.
/// 
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `operator`: Operador condicional en formato string.
/// - `tokens`: Iterador de los tokens restantes en la ejecución.
/// 
/// # Errores
/// Retorna un error si el operador no es reconocido.
pub fn apply_conditional_operation(
    stack: &mut Stack,
    operator: &str,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<(), String> {
    match operator {
        "IF" => handle_if(stack, tokens),
        _ => Err(format!(
            "Error: Operador condicional no reconocido '{}'",
            operator
        )),
    }
}

/// Maneja la ejecución de la estructura condicional `IF ... THEN`.
/// 
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `tokens`: Iterador de los tokens restantes en la ejecución.
/// 
/// # Errores
/// - Retorna un error si la pila está vacía antes de evaluar `IF`.
/// - Retorna un error si no se encuentra el token `THEN`.
fn handle_if(stack: &mut Stack, tokens: &mut std::str::SplitWhitespace) -> Result<(), String> {
    let condition = stack
        .pop()
        .ok_or("Error: La pila está vacía antes de 'IF'")?;

    if condition != 0 {
        while let Some(token) = tokens.next() {
            if token.to_uppercase() == "THEN" {
                return Ok(());
            }
            crate::program::execute_operation(stack, token.to_string())?;
        }
    } else {
        while let Some(token) = tokens.next() {
            if token.to_uppercase() == "THEN" {
                return Ok(());
            }
        }
    }

    Err("Error: Falta 'THEN' en la estructura de control".to_string())
}
