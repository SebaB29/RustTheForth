use crate::stack::Stack;

pub fn apply_conditional_operation(
    stack: &mut Stack,
    operator: &str,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<(), String> {
    match operator {
        "IF" => handle_if(stack, tokens),
        _ => Err(format!(
            "Error: Operador condicional no reconocido '{}'",
            operator
        )),
    }
}

fn handle_if(stack: &mut Stack, tokens: &mut std::str::SplitWhitespace) -> Result<(), String> {
    let condition = stack
        .pop()
        .ok_or("Error: La pila está vacía antes de 'IF'")?;

    if condition != 0 {
        while let Some(token) = tokens.next() {
            if token.to_uppercase() == "THEN" {
                return Ok(());
            }
            crate::program::execute_operation(stack, token.to_string())?;
        }
    } else {
        while let Some(token) = tokens.next() {
            if token.to_uppercase() == "THEN" {
                return Ok(());
            }
        }
    }

    Err("Error: Falta 'THEN' en la estructura de control".to_string())
}
