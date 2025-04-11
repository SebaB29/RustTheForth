use crate::stack::Stack;
use crate::word_definitions::WordMap;

/// Aplica una operación condicional a la pila según el operador recibido.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `operator`: Operador condicional (actualmente solo "IF").
/// - `tokens`: Iterador de los tokens restantes a procesar.
/// - `word_map`: Mapa de palabras definidas por el usuario.
///
/// # Retornos
/// Retorna `Ok(())` si la operación condicional se ejecuta correctamente,
/// o `Err("?"`) si el operador no es reconocido.
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

/// Maneja la ejecución de una estructura condicional `IF ... ELSE ... THEN`.
///
/// Evalúa la condición en la cima de la pila. Si es distinta de cero (true), se ejecuta la rama verdadera;
/// en caso contrario, se ejecuta la rama falsa. El cuerpo de cada rama se obtiene llamando a `parse_conditional_branches`.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `tokens`: Iterador de los tokens restantes a procesar.
/// - `word_map`: Mapa de palabras definidas por el usuario.
///
/// # Retornos
/// - `Ok(())` si la evaluación y ejecución del bloque condicional fue exitosa.
/// - `Err(String)` con:
///     - `"stack-underflow"`: Si no hay suficientes elementos en la pila para obtener la condición.
///     - `"Error: Falta 'THEN' en la estructura de control"`: Si no se encuentra el token `THEN`.
fn handle_if(
    stack: &mut Stack,
    tokens: &mut std::str::SplitWhitespace,
    word_map: &mut WordMap,
) -> Result<(), String> {
    let condition = stack.pop().ok_or("stack-underflow")?;
    let (true_branch, false_branch, then_found) = parse_conditional_branches(tokens)?;

    if !then_found {
        return Err("Error: Falta 'THEN' en la estructura de control".to_string());
    }

    let selected_branch = if condition != 0 {
        true_branch
    } else {
        false_branch
    };

    execute_branch(stack, &selected_branch, word_map)
}

/// Separa y agrupa los tokens correspondientes a las ramas de una estructura condicional `IF ... ELSE ... THEN`.
///
/// Esta función analiza los tokens recibidos y divide el contenido en dos ramas:
/// - `true_branch`: Rama que se ejecuta si la condición es verdadera.
/// - `false_branch`: Rama que se ejecuta si la condición es falsa (si existe `ELSE`).
///
/// # Parámetros
/// - `tokens`: Iterador mutable sobre los tokens restantes del programa.
///
/// # Retornos
/// Una tupla con tres valores:
/// - `Vec<String>`: Tokens de la rama verdadera (`true_branch`).
/// - `Vec<String>`: Tokens de la rama falsa (`false_branch`).
/// - `bool`: Indica si se encontró correctamente el token de cierre `THEN`.
fn parse_conditional_branches(
    tokens: &mut std::str::SplitWhitespace,
) -> Result<(Vec<String>, Vec<String>, bool), String> {
    let mut true_branch = Vec::new();
    let mut false_branch = Vec::new();
    let mut current_branch = &mut true_branch;
    let mut depth = 0;
    let mut then_found = false;

    for token in tokens.by_ref() {
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

    Ok((true_branch, false_branch, then_found))
}

/// Ejecuta una secuencia de tokens correspondiente a una rama condicional.
///
/// Esta función toma los tokens de una rama (`true_branch` o `false_branch`), los une en una sola línea y los ejecuta.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `branch`: Referencia a un slice de `String` que representa los tokens de la rama a ejecutar.
/// - `word_map`: Mapa de definiciones de palabras del usuario.
fn execute_branch(
    stack: &mut Stack,
    branch: &[String],
    word_map: &mut WordMap,
) -> Result<(), String> {
    let joined = branch.join(" ");
    crate::program::execute_operation(stack, joined, word_map)
}
