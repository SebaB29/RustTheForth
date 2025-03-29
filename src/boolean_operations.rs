use crate::stack::Stack;

const FALSE: i16 = 0;
const TRUE: i16 = -1;

pub fn apply_boolean_operation(stack: &mut Stack, operator: &str) {
    match operator {
        "=" => equal(stack),
        "<" => lower_than(stack),
        ">" => greater_than(stack),
        "AND" => and(stack),
        "OR" => or(stack),
        "NOT" => not(stack),
        _ => {}
    }
}

fn equal(stack: &mut Stack) {
    if stack.len() < 2 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value_1 = stack.pop();
    let value_2 = stack.pop();
    stack.push(if value_2 == value_1 { TRUE } else { FALSE });
}

fn lower_than(stack: &mut Stack) {
    if stack.len() < 2 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value_1 = stack.pop();
    let value_2 = stack.pop();
    stack.push(if value_2 < value_1 { TRUE } else { FALSE });
}

fn greater_than(stack: &mut Stack) {
    if stack.len() < 2 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value_1 = stack.pop();
    let value_2 = stack.pop();
    stack.push(if value_2 > value_1 { TRUE } else { FALSE });
}

fn and(stack: &mut Stack) {
    if stack.len() < 2 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value_1 = stack.pop();
    let value_2 = stack.pop();
    stack.push(if (value_2 != Some(FALSE)) && (value_1 != Some(FALSE)) {
        TRUE
    } else {
        FALSE
    });
}

fn or(stack: &mut Stack) {
    if stack.len() < 2 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value_1 = stack.pop();
    let value_2 = stack.pop();
    stack.push(if (value_2 != Some(FALSE)) || (value_1 != Some(FALSE)) {
        TRUE
    } else {
        FALSE
    });
}

fn not(stack: &mut Stack) {
    if stack.len() == 0 {
        println!("No hay suficientes elementos en la Pila.");
        return;
    }

    let value = stack.pop();
    stack.push(if value != Some(FALSE) { FALSE } else { TRUE });
}
