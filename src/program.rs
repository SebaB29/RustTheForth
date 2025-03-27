use crate::arithmetic_operations::apply_operation;
use crate::forth_basic_operations::apply_forth_operation;
use crate::stack::Stack;
use std::env;
use std::fs;

const DEFAULT_STACK_SIZE: usize = 128 * 1024;

pub fn parse_args() -> Result<(String, usize), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Error: Debes especificar un archivo .fth".to_string());
    }

    let filename = args[1].clone();
    let stack_size = parse_stack_size(&args);

    Ok((filename, stack_size))
}

pub fn execute_program(stack_size: usize, filename: String) {
    let mut stack = Stack::new(stack_size);
    match read_file(filename) {
        Ok(content) => {
            execute(&mut stack, content);
        }
        Err(error_msg) => println!("{}", error_msg),
    }
}

fn parse_stack_size(args: &[String]) -> usize {
    for arg in args {
        if arg.starts_with("stack-size=") {
            if let Some(size_str) = arg.split('=').nth(1) {
                if let Ok(size) = size_str.parse::<usize>() {
                    return size;
                }
            }
        }
    }
    DEFAULT_STACK_SIZE
}

fn read_file(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(error_msg) => Err(format!(
            "Error al leer el archivo '{}': {}",
            filename, error_msg
        )),
    }
}

fn execute(stack: &mut Stack, input: String) {
    for token in input.split_whitespace() {
        match token {
            "dup" | "drop" | "swap" | "over" | "rot" => apply_forth_operation(stack, token),
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
    while let Some(value) = stack.pop() {
        println!("{} ", value)
    }
}
