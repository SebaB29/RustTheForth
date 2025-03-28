use crate::stack::Stack;

pub fn apply_operation(stack: &mut Stack, operator: &str) {
    if stack.len() < 2 {
        println!("Error: No hay suficientes elementos en la Pila");
        return;
    }

    match operator {
        "+" => sum(stack),
        "-" => subtraction(stack),
        "*" => multiplication(stack),
        "/" => division(stack),
        _ => {}
    }
}

fn sum(stack: &mut Stack) {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b + a);
    }
}

fn subtraction(stack: &mut Stack) {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b - a);
    }
}

fn multiplication(stack: &mut Stack) {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        stack.push(b * a);
    }
}

fn division(stack: &mut Stack) {
    match (stack.pop(), stack.pop()) {
        (Some(0), Some(_)) => println!("Error: División por cero"),
        (Some(a), Some(b)) => stack.push(b / a),
        _ => {}
    }
}
