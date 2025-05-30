use forth_interpreter::arithmetic_operations::apply_arithmetic_operation;
use forth_interpreter::stack::Stack;

#[cfg(test)]
mod arithmetic_operations_test {
    use super::*;

    fn setup_stack() -> Stack {
        Stack::new(128 * 1024)
    }

    #[test]
    fn test_addition() {
        let mut stack = setup_stack();

        stack.push(2);
        stack.push(3);

        let result = apply_arithmetic_operation(&mut stack, "+");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(5));
    }

    #[test]
    fn test_subtraction() {
        let mut stack = setup_stack();

        stack.push(5);
        stack.push(3);

        let result = apply_arithmetic_operation(&mut stack, "-");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn test_multiplication() {
        let mut stack = setup_stack();

        stack.push(3);
        stack.push(4);

        let result = apply_arithmetic_operation(&mut stack, "*");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(12));
    }

    #[test]
    fn test_division() {
        let mut stack = setup_stack();

        stack.push(12);
        stack.push(4);

        let result = apply_arithmetic_operation(&mut stack, "/");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn test_division_by_zero() {
        let mut stack = setup_stack();

        stack.push(12);
        stack.push(0);

        let result = apply_arithmetic_operation(&mut stack, "/");
        assert_eq!(result, Err("division-by-zero".to_string()));
    }

    #[test]
    fn test_not_enough_elements_for_addition() {
        let mut stack = setup_stack();

        stack.push(2);

        let result = apply_arithmetic_operation(&mut stack, "+");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_not_enough_elements_for_subtraction() {
        let mut stack = setup_stack();

        stack.push(2);

        let result = apply_arithmetic_operation(&mut stack, "-");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_not_enough_elements_for_multiplication() {
        let mut stack = setup_stack();

        stack.push(2);

        let result = apply_arithmetic_operation(&mut stack, "*");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_not_enough_elements_for_division() {
        let mut stack = setup_stack();

        stack.push(2);

        let result = apply_arithmetic_operation(&mut stack, "/");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_chained_arithmetic_operations() {
        let mut stack = setup_stack();

        stack.push(3);
        stack.push(4);
        assert_eq!(apply_arithmetic_operation(&mut stack, "+"), Ok(()));

        stack.push(2);
        assert_eq!(apply_arithmetic_operation(&mut stack, "*"), Ok(()));

        stack.push(5);
        assert_eq!(apply_arithmetic_operation(&mut stack, "-"), Ok(()));

        assert_eq!(stack.pop(), Some(9));
    }
}
