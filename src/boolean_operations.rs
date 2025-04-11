use crate::stack::Stack;

/// Representación de valores booleanos en Forth.
const FALSE: i16 = 0;
const TRUE: i16 = -1;

/// Aplica una operación booleana sobre los elementos superiores de la pila.
///
/// Esta función toma uno o dos valores desde el tope de la pila, dependiendo del operador,
/// aplica la operación booleana especificada y coloca el resultado nuevamente en la pila.
///
/// Las operaciones soportadas son:
/// - `"="`: Compara igualdad entre dos elementos.
/// - `"<"`: Evalúa si el penúltimo valor es menor que el último.
/// - `">"`: Evalúa si el penúltimo valor es mayor que el último.
/// - `"AND"`: Conjunción lógica entre los dos valores superiores.
/// - `"OR"`: Disyunción lógica entre los dos valores superiores.
/// - `"NOT"`: Negación lógica del valor superior.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la cual se realizará la operación.
/// - `operator`: Un `&str` que representa el operador lógico a aplicar.
///
/// # Retorna
///
/// - `Ok(())` si la operación se ejecuta correctamente.
/// - `Err(String)` si:
///   - La pila no contiene suficientes elementos (stack underflow).
///   - El operador no es reconocido.
pub fn apply_boolean_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "=" => equal(stack),
        "<" => lower_than(stack),
        ">" => greater_than(stack),
        "AND" => and(stack),
        "OR" => or(stack),
        "NOT" => not(stack),
        _ => Err("?".to_string()),
    }
}

/// Extrae los dos elementos superiores de la pila y los retorna como una tupla.
///
/// Esta función intenta realizar dos operaciones `pop` sobre la pila. Si ambas son exitosas,
/// retorna los valores extraídos como una tupla `(a, b)`, donde:
/// - `a` es el valor que estaba en la cima.
/// - `b` es el valor que estaba justo debajo.
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila desde donde se extraerán los valores.
///
/// # Retorna
///
/// - `Ok((i16, i16))` si se logran extraer dos valores correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn pop_two_operands(stack: &mut Stack) -> Result<(i16, i16), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        Ok((a, b))
    } else {
        Err("stack-underflow".to_string())
    }
}

/// Compara si los dos valores superiores de la pila son iguales.
///
/// Esta función extrae los dos valores del tope de la pila y verifica si son iguales.
/// Si lo son, empuja `TRUE` (-1) a la pila; de lo contrario, empuja `FALSE` (0).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la comparación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn equal(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_two_operands(stack)?;
    stack.push(if a == b { TRUE } else { FALSE });
    Ok(())
}

/// Compara si el segundo valor desde el tope de la pila es menor que el primero.
///
/// Esta función extrae los dos valores del tope de la pila y evalúa si `b < a`,
/// donde `a` es el valor en el tope de la pila y `b` el siguiente.
/// Si la condición se cumple, empuja `TRUE` (-1) a la pila; de lo contrario, empuja `FALSE` (0).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la comparación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn lower_than(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_two_operands(stack)?;
    stack.push(if b < a { TRUE } else { FALSE });
    Ok(())
}

/// Compara si el segundo valor desde el tope de la pila es mayor que el primero.
///
/// Esta función extrae los dos valores del tope de la pila y evalúa si `b > a`,
/// donde `a` es el valor en el tope de la pila y `b` el siguiente.
/// Si la condición se cumple, empuja `TRUE` (-1) a la pila; de lo contrario, empuja `FALSE` (0).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la comparación.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn greater_than(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_two_operands(stack)?;
    stack.push(if b > a { TRUE } else { FALSE });
    Ok(())
}

/// Realiza la conjunción lógica (AND) entre los dos valores superiores de la pila.
///
/// Esta función extrae dos valores desde el tope de la pila y evalúa la operación lógica
/// `a AND b`, considerando que cualquier valor distinto de `FALSE` (0) es verdadero.
/// Si ambos valores son verdaderos, empuja `TRUE` (-1) a la pila; de lo contrario, empuja `FALSE` (0).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la operación lógica.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn and(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_two_operands(stack)?;
    stack.push(if a != FALSE && b != FALSE {
        TRUE
    } else {
        FALSE
    });
    Ok(())
}

/// Realiza la disyunción lógica (OR) entre los dos valores superiores de la pila.
///
/// Esta función extrae dos valores desde el tope de la pila y evalúa la operación lógica
/// `a OR b`, considerando que cualquier valor distinto de `FALSE` (0) es verdadero.
/// Si al menos uno de los valores es verdadero, empuja `TRUE` (-1) a la pila;
/// de lo contrario, empuja `FALSE` (0).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la operación lógica.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene suficientes elementos.
fn or(stack: &mut Stack) -> Result<(), String> {
    let (a, b) = pop_two_operands(stack)?;
    stack.push(if a != FALSE || b != FALSE {
        TRUE
    } else {
        FALSE
    });
    Ok(())
}

/// Realiza la negación lógica (NOT) del valor superior de la pila.
///
/// Esta función extrae el valor en el tope de la pila y aplica la operación lógica `NOT`.
/// Si el valor es distinto de `FALSE` (0), se considera verdadero y se empuja `FALSE` (0) a la pila.
/// Si el valor es `FALSE` (0), se considera falso y se empuja `TRUE` (-1).
///
/// # Parámetros
///
/// - `stack`: Referencia mutable a la pila sobre la que se realizará la operación lógica.
///
/// # Retorna
///
/// - `Ok(())` si la operación se realiza correctamente.
/// - `Err(String)` con el mensaje `"stack-underflow"` si la pila no contiene al menos un elemento.
fn not(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(a) => {
            stack.push(if a != FALSE { FALSE } else { TRUE });
            Ok(())
        }
        None => Err("stack-underflow".to_string()),
    }
}
