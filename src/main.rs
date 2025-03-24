fn main() {
    let mut stack: Vec<i32> = vec![];

    stack.push(20);
    stack.push(10);

    apply_operation(&mut stack, "+");

    println!("El último elemento de la pila es: {:?}", stack.pop());
}

fn apply_operation(stack: &mut Vec<i32>, operator: &str) {
    match operator {
        "+" => apply_sum(stack),
        // "-" => ,
        // "*" => ,
        // "/" => ,
        _ => println!("Operador no reconocido, operadores admitidos: + - * /"),
    }
}

fn apply_sum(stack: &mut Vec<i32>) {
    let summand_1: Option<i32> = stack.pop();
    let summand_2: Option<i32> = stack.pop();

    match (summand_1, summand_2) {
        (Some(a), Some(b)) => stack.push(a + b),
        _ => println!("Error: No hay suficientes elementos en la pila"),
    }
}
