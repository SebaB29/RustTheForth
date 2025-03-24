fn main() {
    let mut stack: Vec<i32> = vec![];

    stack.push(10);
    stack.push(20);

    // apply_operation(&mut stack, "+");
    apply_operation(&mut stack, "-");

    println!("El último elemento de la pila es: {:?}", stack.pop());
}

fn apply_operation(stack: &mut Vec<i32>, operator: &str) {
    match operator {
        "+" => apply_sum(stack),
        "-" => apply_subtraction(stack),
        // "*" => ,
        // "/" => ,
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn apply_sum(stack: &mut Vec<i32>) {
    let number_1: Option<i32> = stack.pop();
    let number_2: Option<i32> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(a + b),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}

fn apply_subtraction(stack: &mut Vec<i32>) {
    let number_1: Option<i32> = stack.pop();
    let number_2: Option<i32> = stack.pop();

    match (number_1, number_2) {
        (Some(a), Some(b)) => stack.push(a - b),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
