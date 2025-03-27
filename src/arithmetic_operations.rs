use crate::stack::Stack;

pub fn apply_operation(stack: &mut Stack, operator: &str) {
    if stack.len() < 2 {
        println!("Operadores Insuficientes.");
        return;
    }

    match operator {
        "+" => sum(stack),
        "-" => subtraction(stack),
        "*" => multiplication(stack),
        "/" => division(stack),
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn sum(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b + a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn subtraction(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b - a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn multiplication(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b * a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn division(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b / a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
