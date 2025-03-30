use crate::stack::Stack;

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

fn drop(stack: &mut Stack) -> Result<(), String> {
    if stack.pop().is_none() {
        Err("Error: La pila está vacía.".to_string())
    } else {
        Ok(())
    }
}

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
