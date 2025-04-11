use crate::stack::Stack;
use crate::word_definitions::WordMap;

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
    word_map: &mut WordMap,
) -> Result<(), String> {
    match operator {
        "IF" => handle_if(stack, tokens, word_map),
        _ => Err("?".to_string()),
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
fn handle_if(
    stack: &mut Stack,
    tokens: &mut std::str::SplitWhitespace,
    word_map: &mut WordMap,
) -> Result<(), String> {
    let condition = stack.pop().ok_or("stack-underflow")?;

    let mut true_branch = Vec::new();
    let mut false_branch = Vec::new();
    let mut current_branch = &mut true_branch;
    let mut depth = 0;
    let mut then_found = false;

    while let Some(token) = tokens.next() {
        let token_up = token.to_uppercase();

        match token_up.as_str() {
            "IF" => {
                depth += 1;
                current_branch.push(token.to_string());
            }
            "ELSE" if depth == 0 => {
                current_branch = &mut false_branch;
            }
            "THEN" => {
                if depth == 0 {
                    then_found = true;
                    break;
                } else {
                    depth -= 1;
                    current_branch.push(token.to_string());
                }
            }
            _ => {
                current_branch.push(token.to_string());
            }
        }
    }

    if !then_found {
        return Err("Error: Falta 'THEN' en la estructura de control".to_string());
    }

    let selected_branch = if condition != 0 {
        true_branch
    } else {
        false_branch
    };

    // Ejecutamos la rama seleccionada como una línea
    let joined = selected_branch.join(" ");
    crate::program::execute_operation(stack, joined, word_map)?;

    Ok(())
}
