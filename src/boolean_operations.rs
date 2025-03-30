use crate::stack::Stack;

/// Representación de valores booleanos en Forth.
const FALSE: i16 = 0;
const TRUE: i16 = -1;

/// Aplica una operación booleana a la pila.
///
/// # Parámetros
/// - `stack`: Referencia mutable a la pila.
/// - `operator`: Cadena que representa la operación booleana a ejecutar.
/// # Operadores Soportados
/// - `=`  (Igualdad)
/// - `<`  (Menor que)
/// - `>`  (Mayor que)
/// - `AND` (Conjunción lógica)
/// - `OR`  (Disyunción lógica)
/// - `NOT` (Negación lógica)
///
/// # Errores
/// Retorna un error si el operador no es reconocido o si no hay suficientes elementos en la pila.
pub fn apply_boolean_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "=" => equal(stack),
        "<" => lower_than(stack),
        ">" => greater_than(stack),
        "AND" => and(stack),
        "OR" => or(stack),
        "NOT" => not(stack),
        _ => Err("Error: Operador booleano no reconocido".to_string()),
    }
}

/// Compara si los dos valores superiores de la pila son iguales.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn equal(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(if a == b { TRUE } else { FALSE });
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Compara si el penúltimo valor de la pila es menor que el último.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn lower_than(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(if b < a { TRUE } else { FALSE });
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Compara si el penúltimo valor de la pila es mayor que el último.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn greater_than(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(if b > a { TRUE } else { FALSE });
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Realiza la conjunción lógica (AND) entre los dos valores superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn and(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(if a != FALSE && b != FALSE {
                TRUE
            } else {
                FALSE
            });
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Realiza la disyunción lógica (OR) entre los dos valores superiores de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn or(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => {
            stack.push(if a != FALSE || b != FALSE {
                TRUE
            } else {
                FALSE
            });
            Ok(())
        }
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

/// Realiza la negación lógica (NOT) del valor superior de la pila.
///
/// # Errores
/// Retorna un `Err(String)` si no hay suficientes elementos en la pila.
fn not(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(a) => {
            stack.push(if a != FALSE { FALSE } else { TRUE });
            Ok(())
        }
        None => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}
