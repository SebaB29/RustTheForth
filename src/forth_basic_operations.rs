use crate::stack::Stack;

pub fn apply_forth_operation(stack: &mut Stack, operator: &str) {
    match operator {
        "dup" => dup(stack),
        "drop" => drop(stack),
        "swap" => swap(stack),
        "over" => over(stack),
        "rot" => rot(stack),
        _ => {
            println!("Operador no reconocido, operadores admitidos: dup | drop | swap | over | rot")
        }
    }
}

fn dup(stack: &mut Stack) {
    match stack.pop() {
        Some(value) => {
            stack.push(value);
            stack.push(value);
        }
        _ => println!("No hay suficientes elementos en la Pila."),
    }
}

fn drop(stack: &mut Stack) {
    if stack.pop().is_none() {
        println!("La pila está vacía.");
    }
}

fn swap(stack: &mut Stack) {
    match (stack.pop(), stack.pop()) {
        (Some(value_1), Some(value_2)) => {
            stack.push(value_1);
            stack.push(value_2);
        }
        _ => println!("No hay suficientes elementos en la Pila."),
    }
}

fn over(stack: &mut Stack) {
    match (stack.pop(), stack.pop()) {
        (Some(value_1), Some(value_2)) => {
            stack.push(value_2);
            stack.push(value_1);
            stack.push(value_2);
        }
        _ => println!("No hay suficientes elementos en la Pila."),
    }
}

fn rot(stack: &mut Stack) {
    let aux_stack = Stack::new(128 * 1024);
}
