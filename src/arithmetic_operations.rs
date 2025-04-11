use crate::stack::Stack;

/// Aplica una operación aritmética sobre los dos elementos superiores de la pila.
///
/// Esta función toma dos valores desde el tope de la pila, aplica la operación
/// aritmética especificada y coloca el resultado nuevamente en la pila.
///
/// Las operaciones soportadas son:
/// - `"+"`: Suma.
/// - `"-"`: Resta.
/// - `"*"`: Multiplicación.
/// - `"/"`: División.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la cual se realizará la operación.
/// - `operator`: Un `&str` que indica el operador a aplicar (`+`, `-`, `*` o `/`).
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` si:
///   - La pila no contiene suficientes elementos (stack underflow).
///   - Se intenta una división por cero.
///   - El operador no es reconocido.
pub fn apply_arithmetic_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "+" => sum(stack),
        "-" => subtraction(stack),
        "*" => multiplication(stack),
        "/" => division(stack),
        _ => Err("?".to_string()),
    }
}

/// Extrae los dos elementos superiores de la pila y los retorna como una tupla.
///
/// Esta función intenta hacer `pop` dos veces sobre la pila. Si ambas extracciones
/// son exitosas, retorna los valores como una tupla `(a, b)`, donde `a` es el valor
/// que estaba en el tope y `b` el que estaba justo debajo.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila desde la cual se extraerán los valores.
///
/// # Retorna
///
/// - `Ok((i16, i16))` con los dos valores extraídos si la pila tiene al menos dos elementos.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no tiene suficientes elementos.
fn pop_operands(stack: &mut Stack) -> Result<(i16, i16), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        Ok((a, b))
    } else {
        Err("stack-underflow".to_string())
    }
}

/// Realiza la suma de los dos elementos superiores de la pila.
///
/// Esta función extrae los dos valores en el tope de la pila, los suma y
/// coloca el resultado nuevamente en la pila.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se ejecuta la operación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no tiene suficientes elementos.
fn sum(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_operands(stack)?;
    stack.push(b + a);
    Ok(())
}

/// Realiza la resta de los dos elementos superiores de la pila.
///
/// Extrae los dos valores del tope de la pila, realiza la resta `b - a`
/// (donde `a` es el valor en el tope de la pila) y coloca el resultado
/// nuevamente en la pila.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se ejecuta la operación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no tiene suficientes elementos.
fn subtraction(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_operands(stack)?;
    stack.push(b - a);
    Ok(())
}

/// Realiza la multiplicación de los dos elementos superiores de la pila.
///
/// Extrae los dos valores del tope de la pila, realiza la operación `b * a`
/// (donde `a` es el valor en el tope de la pila) y coloca el resultado
/// nuevamente en la pila.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se ejecuta la operación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no tiene suficientes elementos.
fn multiplication(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_operands(stack)?;
    stack.push(b * a);
    Ok(())
}

/// Realiza la división de los dos elementos superiores de la pila.
///
/// Extrae los dos valores del tope de la pila, realiza la operación `b / a`
/// (donde `a` es el valor en el tope de la pila) y coloca el resultado en la pila.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se ejecuta la operación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con:
///   - `"stack-underflow"` si la pila no tiene suficientes elementos.
///   - `"division-by-zero"` si el divisor (`a`) es cero.
fn division(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_operands(stack)?;
    if a == 0 {
        return Err("division-by-zero".to_string());
    }
    stack.push(b / a);
    Ok(())
}
