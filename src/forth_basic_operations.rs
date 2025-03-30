use crate::stack::Stack;

/// Aplica una operación Forth sobre la pila.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
/// * `operator` - El operador Forth que se va a ejecutar (DUP, DROP, SWAP, OVER, ROT).
///
/// # Errores
///
/// Retorna un `Err(String)` si no fue posible realizar la operación.
pub fn apply_forth_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "DUP" => dup(stack),
        "DROP" => drop(stack),
        "SWAP" => swap(stack),
        "OVER" => over(stack),
        "ROT" => rot(stack),
        _ => Err("Error: Operación Forth no reconocida".to_string()),
    }
}

/// Duplica el elemento en la parte superior de la pila.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn dup(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(value) => {
            stack.push(value);
            stack.push(value);
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Elimina el elemento en la parte superior de la pila.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si la pila esta vacía.
fn drop(stack: &mut Stack) -> Result<(), String> {
    if stack.pop().is_none() {
        Err("Error: La pila está vacía.".to_string())
    } else {
        Ok(())
    }
}

/// Intercambia los dos elementos superiores de la pila.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn swap(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(a);
            stack.push(b);
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Duplica el segundo elemento más cercano a la parte superior de la pila y lo coloca en la parte superior.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn over(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(b);
            stack.push(a);
            stack.push(b);
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Rota cíclicamente los elementos de la pila, moviendo el elemento en la base de la pila a la cima.
///
/// # Argumentos
///
/// * `stack` - La pila sobre la cual se realiza la operación.
///
/// # Errores
///
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn rot(stack: &mut Stack) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Error: No hay suficientes elementos en la pila".to_string());
    }

    let mut temp_stack = Vec::new();
    while let Some(value) = stack.pop() {
        temp_stack.push(value);
    }

    if let Some(bottom) = temp_stack.pop() {
        while let Some(value) = temp_stack.pop() {
            stack.push(value);
        }

        stack.push(bottom);
    }

    Ok(())
}
