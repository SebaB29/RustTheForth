use crate::stack::Stack;

pub fn apply_arithmetic_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "+" => sum(stack),
        "-" => subtraction(stack),
        "*" => multiplication(stack),
        "/" => division(stack),
        _ => Err("Error: Operador no reconocido".to_string()),
    }
}

fn sum(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        Ok(stack.push(b + a))
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

fn subtraction(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        Ok(stack.push(b - a))
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

fn multiplication(stack: &mut Stack) -> Result<(), String> {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        Ok(stack.push(b * a))
    } else {
        Err("Error: No hay suficientes elementos en la pila".to_string())
    }
}

fn division(stack: &mut Stack) -> Result<(), String> {
    match (stack.pop(), stack.pop()) {
        (Some(0), Some(_)) => Err("Error: División por cero".to_string()),
        (Some(a), Some(b)) => Ok(stack.push(b / a)),
        _ => Err("Error: No hay suficientes elementos en la pila".to_string()),
    }
}
