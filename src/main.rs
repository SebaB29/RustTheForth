fn main() {
    let mut stack: Vec<i16> = vec![];

    stack.push(20);
    stack.push(10);

    // apply_operation(&mut stack, "+");
    // apply_operation(&mut stack, "-");
    // apply_operation(&mut stack, "*");
    apply_operation(&mut stack, "/");

    println!("El último elemento de la pila es: {:?}", stack.pop());
}

fn apply_operation(stack: &mut Vec<i16>, operator: &str) {
    match operator {
        "+" => apply_sum(stack),
        "-" => apply_subtraction(stack),
        "*" => apply_multiplication(stack),
        "/" => apply_division(stack),
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn apply_sum(stack: &mut Vec<i16>) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b + a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_subtraction(stack: &mut Vec<i16>) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b - a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_multiplication(stack: &mut Vec<i16>) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b * a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_division(stack: &mut Vec<i16>) {
    let number_1: Option<i16> = stack.pop();
    let number_2: Option<i16> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(b / a),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
