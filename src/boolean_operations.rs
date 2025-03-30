use crate::stack::Stack;

const FALSE: i16 = 0;
const TRUE: i16 = -1;

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

fn equal(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Ok(stack.push(if a == b { TRUE } else { FALSE })),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

fn lower_than(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Ok(stack.push(if b < a { TRUE } else { FALSE })),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

fn greater_than(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Ok(stack.push(if b > a { TRUE } else { FALSE })),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

fn and(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Ok(stack.push(if a != FALSE && b != FALSE {
            TRUE
        } else {
            FALSE
        })),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

fn or(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Ok(stack.push(if a != FALSE || b != FALSE {
            TRUE
        } else {
            FALSE
        })),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}

fn not(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(a) => Ok(stack.push(if a != FALSE { FALSE } else { TRUE })),
        None => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}
