use forth_interpreter::boolean_operations::apply_boolean_operation;
use forth_interpreter::stack::Stack;

#[cfg(test)]
mod boolean_operations_test {
    use super::*;

    fn setup_stack() -> Stack {
        Stack::new(128 * 1024)
    }

    #[test]
    fn test_equal() {
        let mut stack = setup_stack();

        stack.push(5);
        stack.push(5);

        let result = apply_boolean_operation(&mut stack, "=");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_lower_than() {
        let mut stack = setup_stack();

        stack.push(3);
        stack.push(5);

        let result = apply_boolean_operation(&mut stack, "<");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_greater_than() {
        let mut stack = setup_stack();

        stack.push(5);
        stack.push(3);

        let result = apply_boolean_operation(&mut stack, ">");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_and() {
        let mut stack = setup_stack();

        stack.push(-1);
        stack.push(-1);

        let result = apply_boolean_operation(&mut stack, "AND");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_or() {
        let mut stack = setup_stack();

        stack.push(0);
        stack.push(-1);

        let result = apply_boolean_operation(&mut stack, "OR");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_not() {
        let mut stack = setup_stack();

        stack.push(-1);

        let result = apply_boolean_operation(&mut stack, "NOT");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_not_false() {
        let mut stack = setup_stack();

        stack.push(0);

        let result = apply_boolean_operation(&mut stack, "NOT");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(-1));
    }

    #[test]
    fn test_equal_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_boolean_operation(&mut stack, "=");
        assert_eq!(
            result,
            Err("Error: No hay suficientes elementos en la pila".to_string())
        );
    }

    #[test]
    fn test_and_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_boolean_operation(&mut stack, "AND");
        assert_eq!(
            result,
            Err("Error: No hay suficientes elementos en la pila".to_string())
        );
    }

    #[test]
    fn test_invalid_boolean_operator() {
        let mut stack = setup_stack();

        let result = apply_boolean_operation(&mut stack, "INVALID_OP");
        assert_eq!(
            result,
            Err("Error: Operador booleano no reconocido".to_string())
        );
    }
}
