mod arithmetic_operations;
mod file_handling;
mod forth_basic_operations;
mod program;
mod stack;

fn main() {
    match program::parse_args() {
        Ok((filename, stack_size)) => program::execute_program(stack_size, filename),
        Err(error_msg) => println!("{}", error_msg),
    }
}
