mod arithmetic_operations;
mod boolean_operations;
mod file_handling;
mod forth_basic_operations;
mod output_operations;
mod program;
mod stack;

fn main() {
    match program::parse_args() {
        Ok((filename, stack_size)) => {
            if let Err(error_msg) = program::execute_program(stack_size, filename) {
                println!("{}", error_msg);
            }
        }
        Err(error_msg) => println!("{}", error_msg),
    }
}
