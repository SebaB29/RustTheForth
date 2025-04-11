use forth_interpreter::conditional_operations::apply_conditional_operation;
use forth_interpreter::stack::Stack;
use std::collections::HashMap;

#[cfg(test)]
mod conditional_operations_test {
    use super::*;

    fn setup_stack() -> Stack {
        Stack::new(128 * 1024)
    }

    #[test]
    fn test_if_true_then_executes_branch() {
        let mut stack = setup_stack();
        let input = "42 THEN";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        stack.push(-1);
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_ok());
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn test_if_false_then_skips_branch() {
        let mut stack = setup_stack();
        let input = "42 THEN";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        stack.push(0);
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_ok());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_if_else_true_executes_if_branch() {
        let mut stack = setup_stack();
        let input = "10 ELSE 20 THEN";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        stack.push(1);
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_ok());
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn test_if_else_false_executes_else_branch() {
        let mut stack = setup_stack();
        let input = "10 ELSE 20 THEN";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        stack.push(0);
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_ok());
        assert_eq!(stack.pop(), Some(20));
    }

    #[test]
    fn test_missing_then_should_fail() {
        let mut stack = setup_stack();
        let input = "42";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        stack.push(1);
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_err());
    }

    #[test]
    fn test_else_without_if_should_fail() {
        let mut stack = setup_stack();
        let input = "ELSE 99 THEN";
        let mut tokens = input.split_whitespace();
        let mut definitions = HashMap::new();

        // Mal uso: no hay IF
        let result = apply_conditional_operation(&mut stack, "IF", &mut tokens, &mut definitions);
        assert!(result.is_err());
    }
}
