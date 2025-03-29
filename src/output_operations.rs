use crate::stack::Stack;

pub fn apply_output_operation(stack: &mut Stack, operator: &str) {
    match operator {
        "CR" => println!(),
        "." => point(stack),
        _ => {}
    }
}

fn point(stack: &mut Stack) {
    if let Some(value) = stack.pop() {
        println!("{}", value);
    }
}
