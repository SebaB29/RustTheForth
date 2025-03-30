use crate::stack::Stack;

pub fn apply_output_operation(stack: &mut Stack, operator: &str) -> Result<(), String> {
    match operator {
        "CR" => Ok(println!()),
        "." => point(stack),
        _ => Err("Error: Operación de salida no reconocida".to_string()),
    }
}

fn point(stack: &mut Stack) -> Result<(), String> {
    match stack.pop() {
        Some(value) => Ok(println!("{}", value)),
        _ => Err("Error: No hay elementos en la pila para imprimir".to_string()),
    }
}
