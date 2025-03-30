use crate::stack::Stack;

/// Aplica una operación aritmética a la pila.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila donde se aplicará la operación.
/// - `operator`: Un `&str` que representa la operación aritmética a ejecutar.
///
/// # Operaciones soportadas
/// - `"+"` (suma)
/// - `"-"` (resta)
/// - `"*"` (multiplicación)
/// - `"/"` (división)
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
pub fn apply_arithmetic_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "+" => sum(stack),
        "-" => subtraction(stack),
        "*" => multiplication(stack),
        "/" => division(stack),
        _ => Err("Error: Operador no reconocido".to_string()),
    }
}

/// Realiza la suma de los dos elementos superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn sum(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b + a);
        Ok(())
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

/// Realiza la resta de los dos elementos superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn subtraction(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b - a);
        Ok(())
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

/// Realiza la multiplicación de los dos elementos superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn multiplication(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b * a);
        Ok(())
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

/// Realiza la división de los dos elementos superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si el divisor es cero o si no hay suficientes elementos en la pila.
fn division(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(0), Some(_)) => Err("Error: División por cero".to_string()),
        (Some(a), Some(b)) => {
            stack.push(b / a);
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}
