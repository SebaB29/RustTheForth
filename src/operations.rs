use crate::stack::Stack; // Importa Stack desde el módulo stack

pub fn apply_operation(stack: &mut Stack, operator: &str) {
    if stack.len() < 2 {
        println!("Operadores Insuficientes.");
        return;
    }

    match operator {
        "+" => apply_sum(stack),
        "-" => apply_subtraction(stack),
        "*" => apply_multiplication(stack),
        "/" => apply_division(stack),
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn apply_sum(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b + a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_subtraction(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b - a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_multiplication(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b * a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_division(stack: &mut Stack) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b / a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
