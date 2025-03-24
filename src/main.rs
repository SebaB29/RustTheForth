mod operations;
mod stack;

use operations::apply_operation;
use stack::Stack;

fn main() {
    let mut stack = Stack::new(128 * 1024);
    let input = "25 10 + 3 * CR .";
    execute(&mut stack, input);
}

fn execute(stack: &mut Stack, input: &str) {
    for token in input.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => apply_operation(stack, token),
            "CR" => println!(),
            "." => {
                if let Some(value) = stack.pop() {
                    println!("{}", value);
                }
            }
            _ => {
                if let Ok(number) = token.parse::<i16>() {
                    stack.push(number);
                } else {
                    println!("Error: Token no reconocido '{}'", token);
                }
            }
        }
    }
}
