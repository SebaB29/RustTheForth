use forth_interpreter::forth_basic_operations::apply_forth_operation;
use forth_interpreter::stack::Stack;

#[cfg(test)]
mod forth_operations_test {
    use super::*;

    fn setup_stack() -> Stack {
        Stack::new(128 * 1024)
    }

    #[test]
    fn test_dup() {
        let mut stack = setup_stack();

        stack.push(5);

        let result = apply_forth_operation(&mut stack, "DUP");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(5));
    }

    #[test]
    fn test_drop() {
        let mut stack = setup_stack();

        stack.push(10);

        let result = apply_forth_operation(&mut stack, "DROP");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_swap() {
        let mut stack = setup_stack();

        stack.push(1);
        stack.push(2);

        let result = apply_forth_operation(&mut stack, "SWAP");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn test_over() {
        let mut stack = setup_stack();

        stack.push(3);
        stack.push(4);

        let result = apply_forth_operation(&mut stack, "OVER");
        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn test_rot() {
        let mut stack = setup_stack();

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        let result = apply_forth_operation(&mut stack, "ROT");

        assert_eq!(result, Ok(()));
        assert_eq!(stack.pop(), Some(1)); // El elemento que estaba en la base ahora está en la cima
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn test_dup_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_forth_operation(&mut stack, "DUP");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_drop_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_forth_operation(&mut stack, "DROP");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_swap_not_enough_elements() {
        let mut stack = setup_stack();

        stack.push(1);

        let result = apply_forth_operation(&mut stack, "SWAP");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_over_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_forth_operation(&mut stack, "OVER");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_rot_not_enough_elements() {
        let mut stack = setup_stack();

        let result = apply_forth_operation(&mut stack, "ROT");
        assert_eq!(result, Err("stack-underflow".to_string()));
    }
}
