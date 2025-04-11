use crate::stack::Stack;

/// Aplica una operación del conjunto básico de instrucciones Forth sobre la pila.
///
/// # Operadores soportados
/// - `DUP`: Duplica el elemento en el tope de la pila.
/// - `DROP`: Elimina el elemento en el tope de la pila.
/// - `SWAP`: Intercambia los dos elementos superiores de la pila.
/// - `OVER`: Copia el penúltimo elemento y lo coloca en el tope.
/// - `ROT`: Rota los elementos de la pila, colocando el elemento de la base en el tope.
/// 
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
/// - `operator`: Operador en formato string que representa la operación Forth a ejecutar.
///
/// # Retorna
/// - `Ok(())` si la operación se ejecuta correctamente.
/// - `Err(String)` si el operador no es reconocido.
pub fn apply_forth_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "DUP" => dup(stack),
        "DROP" => drop(stack),
        "SWAP" => swap(stack),
        "OVER" => over(stack),
        "ROT" => rot(stack),
        _ => Err("?".to_string()),
    }
}

/// Duplica el elemento en la cima de la pila.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorna
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si la pila está vacía (stack underflow).
fn dup(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(value) => {
            stack.push(value);
            stack.push(value);
            Ok(())
        }
        _ => Err("stack-underflow".to_string()),
    }
}

/// Elimina el elemento en la cima de la pila.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorna
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si la pila está vacía (stack underflow).
fn drop(stack: &mut Stack) -> Result<(), String> {
    if stack.pop().is_none() {
        Err("stack-underflow".to_string())
    } else {
        Ok(())
    }
}

/// Intercambia los dos elementos superiores de la pila.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorna
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si la pila está vacía (stack underflow).
fn swap(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(a);
            stack.push(b);
            Ok(())
        }
        _ => Err("stack-underflow".to_string()),
    }
}

/// Copia el penúltimo elemento de la pila y lo empuja al tope.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorna
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si la pila está vacía (stack underflow).
fn over(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(b);
            stack.push(a);
            stack.push(b);
            Ok(())
        }
        _ => Err("stack-underflow".to_string()),
    }
}

/// Mueve el elemento de la base de la pila al tope.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila de datos.
///
/// # Retorno
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si la pila está vacía (stack underflow).
fn rot(stack: &mut Stack) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("stack-underflow".to_string());
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
